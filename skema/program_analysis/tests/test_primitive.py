# import json               NOTE: json and Path aren't used right now,
# from pathlib import Path        but will be used in the future
from skema.program_analysis.multi_file_ingester import process_file_system
from skema.gromet.fn import (
    GrometFNModuleCollection,
    FunctionType
)
import ast

from skema.program_analysis.CAST.pythonAST import py_ast_to_cast
from skema.program_analysis.CAST2FN.model.cast import SourceRef
from skema.program_analysis.CAST2FN import cast
from skema.program_analysis.CAST2FN.cast import CAST
from skema.program_analysis.run_ann_cast_pipeline import ann_cast_pipeline

# NOTE: these examples are very trivial for the realm of recursion
#       more complex ones will follow later as needed

def primitive1():
    return """
x = 10
y = 5
z = min(x,y)
    """

def primitive2():
    return """
def foo(x,y):
    z = min(x,y) + y
    return z

foo(10,2)
    """

def primitive3():
    return """
min(10,5)
    """

def generate_gromet(test_file_string):
    # use ast.Parse to get Python AST
    contents = ast.parse(test_file_string)

    # use Python to CAST
    line_count = len(test_file_string.split("\n"))
    convert = py_ast_to_cast.PyASTToCAST("temp")
    C = convert.visit(contents, {}, {})
    C.source_refs = [SourceRef("temp", None, None, 1, line_count)]
    out_cast = cast.CAST([C], "python")

    # use AnnCastPipeline to create GroMEt
    gromet = ann_cast_pipeline(out_cast, gromet=True, to_file=False, from_obj=True)

    return gromet

def test_primitive1():
    exp_gromet = generate_gromet(primitive1())
    
    base_fn = exp_gromet.fn

    assert len(base_fn.b) == 1
    assert len(base_fn.bf) == 3

    assert len(base_fn.pif) == 2
    assert len(base_fn.pof) == 3
    assert len(base_fn.wff) == 2

    assert base_fn.bf[2].function_type == FunctionType.LANGUAGE_PRIMITIVE
    
    assert base_fn.pif[0].box == 3
    assert base_fn.pif[1].box == 3
    assert base_fn.pof[2].box == 3

    assert base_fn.wff[0].src == 1 and base_fn.wff[0].tgt == 1
    assert base_fn.wff[1].src == 2 and base_fn.wff[1].tgt == 2


def test_primitive2():
    exp_gromet = generate_gromet(primitive2())
    
    base_fn = exp_gromet.fn
    primitive_fn = exp_gromet.fn_array[1]
    add_fn = exp_gromet.fn_array[2]

    assert len(base_fn.b) == 1

    assert base_fn.pif[0].box == 1 
    assert base_fn.pif[1].box == 1 

    assert base_fn.pof[0].box == 2 
    assert base_fn.pof[1].box == 3 

    assert base_fn.wff[0].src == 1 and base_fn.wff[0].tgt == 1
    assert base_fn.wff[1].src == 2 and base_fn.wff[1].tgt == 2

    ##############################################
    assert len(primitive_fn.opi) == 2
    assert len(primitive_fn.opo) == 1

    assert len(primitive_fn.bf) == 1

    assert len(primitive_fn.pif) == 2
    assert len(primitive_fn.pof) == 1

    assert len(primitive_fn.wfopi) == 2

    assert primitive_fn.bf[0].function_type == FunctionType.LANGUAGE_PRIMITIVE

    assert primitive_fn.wfopi[0].src == 1 and primitive_fn.wfopi[0].tgt == 1
    assert primitive_fn.wfopi[1].src == 2 and primitive_fn.wfopi[1].tgt == 2

    assert primitive_fn.wfopo[0].src == 1 and primitive_fn.wfopo[0].tgt == 1

    ##############################################
    assert len(add_fn.opi) == 2
    assert len(add_fn.opo) == 1
    
    assert len(add_fn.bf) == 2
    assert len(add_fn.pif) == 4
    assert len(add_fn.pof) == 2

    assert len(add_fn.wfopi) == 3
    assert len(add_fn.wff) == 1
    assert len(add_fn.wfopo) == 1

    # Check bfs
    assert add_fn.bf[0].body == 2
    assert add_fn.bf[1].function_type == FunctionType.LANGUAGE_PRIMITIVE

    # Check wiring
    assert add_fn.wfopi[0].src == 1 and add_fn.wfopi[0].tgt == 1
    assert add_fn.wfopi[1].src == 2 and add_fn.wfopi[1].tgt == 2
    assert add_fn.wfopi[2].src == 4 and add_fn.wfopi[2].tgt == 2
    
    assert add_fn.wff[0].src == 3 and add_fn.wfopi[0].tgt == 1
    
    assert add_fn.wfopo[0].src == 1 and add_fn.wfopo[0].tgt == 2


def test_primitive3():
    exp_gromet = generate_gromet(primitive3())
    
    base_fn = exp_gromet.fn
    primitive_fn = exp_gromet.fn_array[0]

    assert len(base_fn.b) == 1
    assert len(base_fn.bf) == 1

    assert primitive_fn.bf[0].function_type == FunctionType.LANGUAGE_PRIMITIVE
    assert primitive_fn.bf[1].function_type == FunctionType.LITERAL
    assert primitive_fn.bf[2].function_type == FunctionType.LITERAL

    assert primitive_fn.opo[0].box == 1
    
    assert primitive_fn.pif[0].box == 1
    assert primitive_fn.pif[1].box == 1

    assert primitive_fn.pof[0].box == 1
    assert primitive_fn.pof[1].box == 2
    assert primitive_fn.pof[2].box == 3

    assert primitive_fn.wff[0].src == 1 and primitive_fn.wff[0].tgt == 2
    assert primitive_fn.wff[1].src == 2 and primitive_fn.wff[1].tgt == 3
    assert primitive_fn.wfopo[0].src == 1 and primitive_fn.wfopo[0].tgt == 1


def test_primitive():
    test_primitive1()
    test_primitive2()
    test_primitive3()

    return