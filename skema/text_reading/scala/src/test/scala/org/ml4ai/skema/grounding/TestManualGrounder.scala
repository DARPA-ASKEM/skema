package org.ml4ai.skema.grounding

import com.typesafe.config.ConfigFactory
import org.clulab.processors.fastnlp.FastNLPProcessor
import org.ml4ai.skema.test.Test
import org.ml4ai.skema.text_reading.grounding.ManualGrounder
import org.scalatest.OptionValues._

class TestManualGrounder extends Test {

  // Lazily load the grounder. We assume it's state and behavior is immutable
  // So we can build it once and reuse it as necessary in the suite

  val manualGrounder: ManualGrounder = {
    val config = ConfigFactory.load().getConfig("Grounding")
    val domainConfig = config.getConfig(config.getString("domain"))
    val manualEntries = domainConfig.getString("manualGroundings")

    // Set alpha to 1.0 to bypass the edit distance algo for now
    ManualGrounder.fromFileOrResource(manualEntries, new FastNLPProcessor(withChunks = false, internStrings = false))
  }

  "Manual grounder" should s"ground solar flare to the grounding concept with id tr:001" ignore {
    val groundedConcept = manualGrounder.ground("solar flare")
    groundedConcept.value.id should be ("tr:001")
  }

  ignore should "also ground solar flares (in plural) to the grounding concept with id tr:001, because it is a lemma based match" in {
    val groundedConcept = manualGrounder.ground("solar flares")
    groundedConcept.value.id should be("tr:001")
  }

  ignore should "not ground missing item to any grounding concept" in {
    val groundedConcept = manualGrounder ground "missing item"
    groundedConcept shouldBe empty
  }

}
