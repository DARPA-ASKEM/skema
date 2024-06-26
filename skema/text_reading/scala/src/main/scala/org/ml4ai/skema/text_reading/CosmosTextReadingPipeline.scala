package org.ml4ai.skema.text_reading

import org.apache.poi.openxml4j.exceptions.InvalidOperationException
import org.clulab.odin.Mention
import org.clulab.pdf2txt.Pdf2txt
import org.clulab.pdf2txt.common.pdf.TextConverter
import org.clulab.pdf2txt.languageModel.GigawordLanguageModel
import org.clulab.pdf2txt.preprocessor._
import org.clulab.processors.Processor
import org.ml4ai.skema.text_reading.attachments.MentionLocationAttachment
import org.ml4ai.skema.text_reading.data.CosmosJsonDataLoader
import org.ml4ai.skema.text_reading.grounding.Grounder
import org.ml4ai.skema.text_reading.scenario_context.ContextEngine
import org.ml4ai.skema.text_reading.scenario_context.openai.DecoderContextEngine

import scala.util.{Success, Try}
//import org.ml4ai.skema.text_reading.scenario_context.openai.ContextEngine
import org.ml4ai.skema.text_reading.scenario_context.{HeuristicContextEngine, CosmosOrderer}
import org.ml4ai.skema.text_reading.serializer.SkemaJSONSerializer

import scala.collection.mutable.ArrayBuffer

class CosmosTextReadingPipeline(contextWindowSize: Int, contextEngineType:String, processorOpt: Option[Processor] = None, odinEngineOpt: Option[OdinEngine] = None, grounderOpt: Option[Grounder] = None )
    extends TextReadingPipeline(processorOpt, odinEngineOpt, grounderOpt) {

  // PDF converted to fix pdf tokenization artifacts
  private val pdfConverter = new TextConverter()
  private val languageModel = GigawordLanguageModel()
  private val preprocessors = Array(
    new LinePreprocessor(),
    new ParagraphPreprocessor(),
    new UnicodePreprocessor(),
    new CasePreprocessor(CasePreprocessor.defaultCutoff),
    new NumberPreprocessor(NumberPreprocessor.Parameters()),
    new LigaturePreprocessor(languageModel),
    new LineBreakPreprocessor(languageModel),
    new WordBreakByHyphenPreprocessor(),
    // new WordBreakBySpacePreprocessor(languageModel) // This is by default NeverLanguageModel.
  )
  val pdf2txt = new Pdf2txt(pdfConverter, preprocessors)
  ////



  // cosmos stores information about each block on each pdf page
  // for each block, we load the text (content) and the location of the text (page_num and block order/index on the page)
  val loader = new CosmosJsonDataLoader

  private val jsonSeparator  = "<::>"

  /**
    * Runs the textReadingPipeline over a cosmos file
    * @param jsonPath Path to the json file to annotate
    * @return Mentions extracted by the TR textReadingPipeline
    */
  def extractMentionsFromTextsAndLocations(textsAndLocations: Seq[String]): Seq[Mention] = {

    //TODO: Make this interpretable
    val textsAndFilenames = textsAndLocations.map(_.split(jsonSeparator).slice(0, 2).mkString(jsonSeparator))
    val locations = textsAndLocations.map(_.split(jsonSeparator).takeRight(2).mkString(jsonSeparator)) //location = pageNum::blockIdx

    // extract mentions form each text block
    val mentions =
      (for (tf <- textsAndFilenames) yield Try {
        val Array(rawText, filename) = tf.split(jsonSeparator)
        // Extract mentions and apply grounding

        val text = pdf2txt.process(rawText, maxLoops = 1)
        this.extractMentions(text, Some(filename))._2

      }) collect {
        case Success(ms) => ms
      }

    // store location information from cosmos as an attachment for each mention
    val menWInd = mentions.zipWithIndex
    val mentionsWithLocations = new ArrayBuffer[Mention]()

    for (tuple <- menWInd) {
      // get page and block index for each block; cosmos location information will be the same for all the mentions within one block
      val menInTextBlocks = tuple._1
      val id = tuple._2
      val location = locations(id).split(jsonSeparator).map(loc => loc.split(",").map(_.toInt)) //(_.toDouble.toInt)
      val pageNum = location.head
      val blockIdx = location.last

      for (m <- menInTextBlocks) {
        val filename = m.document.id.getOrElse("unknown_file")
        val newMen = m.withAttachment(new MentionLocationAttachment(filename, pageNum, blockIdx, "MentionLocation"))
        mentionsWithLocations.append(newMen)
      }
    }

    // Resolve scenario context
    val cosmosOrderer = new CosmosOrderer(mentionsWithLocations)
    val scenarioContextEngine: ContextEngine = {
      if(contextEngineType.toLowerCase() == "heuristic")
        new HeuristicContextEngine(windowSize = contextWindowSize, mentionsWithLocations, cosmosOrderer)
      else if(contextEngineType.toLowerCase() == "decoder")
        new DecoderContextEngine(contextWindowSize, mentionsWithLocations, cosmosOrderer)
      else
        throw new InvalidOperationException("Undefined type of context engine")
    }
    val mentionsWithScenarioContext = mentionsWithLocations map scenarioContextEngine.resolveContext
    mentionsWithScenarioContext
  }

  def extractMentionsFromCosmosJson(jsonPath: String): Seq[Mention] = {
    logger.info(s"Started annotation of $jsonPath")

    val textsAndLocations = loader.loadFile(jsonPath)
    val result = extractMentionsFromTextsAndLocations(textsAndLocations)

    logger.info(s"Finished annotation of $jsonPath")
    result
  }

    /**
    * Extracts the mentions and serializes them into a json string
    * @param jsonPath Path to the json file to annotate
    * @return string with the json representation of the extractions and the document annotations
    */
  def extractMentionsFromJsonAndSerialize(jsonPath: String): String =
      ujson.write(SkemaJSONSerializer.serializeMentions(this.extractMentionsFromCosmosJson(jsonPath)))

  def extractMentionsFromJsonAndSerialize(ujsonValue: ujson.Js): ujson.Value = {
    val textsAndLocations = loader.loadJson(ujsonValue)
    val mentions = extractMentionsFromTextsAndLocations(textsAndLocations)
    val result = SkemaJSONSerializer.serializeMentions(mentions)

    result
  }
}
