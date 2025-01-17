from typing import List, Dict
from skema.program_analysis.CAST2FN.model.cast import SourceRef
from skema.program_analysis.CAST.matlab.tokens import OTHER_TOKENS, KEYWORDS

from tree_sitter import Node

CONTROL_CHARACTERS = OTHER_TOKENS
        
class NodeHelper():
    def __init__(self, source: str, source_file_name: str):
        self.source = source
        self.source_file_name = source_file_name

    def get_source_ref(self, node: Node) -> SourceRef:
        """Given a node and file name, return a CAST SourceRef object."""
        row_start, col_start = node.start_point
        row_end, col_end = node.end_point
        return SourceRef(self.source_file_name, col_start, col_end, row_start, row_end)

    def get_identifier(self, node: Node) -> str:
        """Given a node, return the identifier it represents. ie. The code between node.start_point and node.end_point"""
        line_num = 0
        column_num = 0
        in_identifier = False
        identifier = ""
        for i, char in enumerate(self.source):
            if line_num == node.start_point[0] and column_num == node.start_point[1]:
                in_identifier = True
            elif line_num == node.end_point[0] and column_num == node.end_point[1]:
                break

            if char == "\n":
                line_num += 1
                column_num = 0
            else:
                column_num += 1

            if in_identifier:
                identifier += char

        return identifier

def get_first_child_by_type(node: Node, child_type: str):
    """ Return the first node child with matching type """
    for child in node.children:
        if child.type == child_type:
            return child

    return None

def get_children_by_types(node: Node, types: List):
    """Takes in a node and a list of types as inputs and returns all children matching those types. Otherwise, return an empty list"""
    return [child for child in node.children if child.type in types]

def get_control_children(node: Node):
    """ return node children with control character types """
    return get_children_by_types(node, CONTROL_CHARACTERS)

def get_keyword_children(node: Node):
    """ return node children with Tree-sitter syntax keyword types """
    return get_children_by_types(node, KEYWORDS)
