# coding: utf-8

# flake8: noqa
"""
    GroMEt Metadata spec

    Grounded Model Exchange (GroMEt) Metadata schema specification  __Using Swagger to Generate Class Structure__  To automatically generate Python or Java models corresponding to this document, you can use [swagger-codegen](https://swagger.io/tools/swagger-codegen/). We can use this to generate client code based off of this spec that will also generate the class structure.  1. Install via the method described for your operating system    [here](https://github.com/swagger-api/swagger-codegen#Prerequisites).    Make sure to install a version after 3.0 that will support openapi 3. 2. Run swagger-codegen with the options in the example below.    The URL references where the yaml for this documentation is stored on    github. Make sure to replace CURRENT_VERSION with the correct version.    (The current version is `0.1.4`.)    To generate Java classes rather, change the `-l python` to `-l java`.    Change the value to the `-o` option to the desired output location.    ```    swagger-codegen generate -l python -o ./client -i https://raw.githubusercontent.com/ml4ai/automates-v2/master/docs/source/gromet_metadata_v{CURRENT_VERSION}.yaml    ``` 3. Once it executes, the client code will be generated at your specified    location.    For python, the classes will be located in    `$OUTPUT_PATH/swagger_client/models/`.    For java, they will be located in    `$OUTPUT_PATH/src/main/java/io/swagger/client/model/`  If generating GroMEt Metadata schema data model classes in SKEMA (AutoMATES), then after generating the above, follow the instructions here: ``` <automates>/automates/model_assembly/gromet/metadata/README.md ```   # noqa: E501

    OpenAPI spec version: 0.1.6
    Contact: claytonm@arizona.edu
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

from __future__ import absolute_import

# import models into model package
from skema.gromet.metadata.bibjson import Bibjson
from skema.gromet.metadata.code_file_reference import CodeFileReference
from skema.gromet.metadata.equation_definition import EquationDefinition
from skema.gromet.metadata.equation_extraction import EquationExtraction
from skema.gromet.metadata.equation_literal_value import EquationLiteralValue
from skema.gromet.metadata.gromet_creation import GrometCreation
from skema.gromet.metadata.literal_value import LiteralValue
from skema.gromet.metadata.metadata import Metadata
from skema.gromet.metadata.program_analysis_record_bookkeeping import ProgramAnalysisRecordBookkeeping
from skema.gromet.metadata.provenance import Provenance
from skema.gromet.metadata.source_code_bool_and import SourceCodeBoolAnd
from skema.gromet.metadata.source_code_bool_or import SourceCodeBoolOr
from skema.gromet.metadata.source_code_collection import SourceCodeCollection
from skema.gromet.metadata.source_code_comment import SourceCodeComment
from skema.gromet.metadata.source_code_data_type import SourceCodeDataType
from skema.gromet.metadata.source_code_loop_init import SourceCodeLoopInit
from skema.gromet.metadata.source_code_loop_update import SourceCodeLoopUpdate
from skema.gromet.metadata.source_code_port_default_val import SourceCodePortDefaultVal
from skema.gromet.metadata.source_code_port_keyword_arg import SourceCodePortKeywordArg
from skema.gromet.metadata.source_code_reference import SourceCodeReference
from skema.gromet.metadata.text_description import TextDescription
from skema.gromet.metadata.text_extraction import TextExtraction
from skema.gromet.metadata.text_extraction_metadata import TextExtractionMetadata
from skema.gromet.metadata.text_grounding import TextGrounding
from skema.gromet.metadata.text_literal_value import TextLiteralValue
from skema.gromet.metadata.text_units import TextUnits
from skema.gromet.metadata.textual_document_collection import TextualDocumentCollection
from skema.gromet.metadata.textual_document_reference import TextualDocumentReference