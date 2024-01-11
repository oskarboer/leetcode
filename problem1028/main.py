# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

def rec(root: Optional[TreeNode], min_parent: int, max_parent: int) -> int:
    l_max, r_max, self_max = 0, 0, 0
    if root != None:
        self_max = max(abs(root.val - min_parent), abs(root.val - max_parent))
        min_parent, max_parent = min(
            root.val, min_parent), max(root.val, max_parent)

        if root.left != None:
            l_max = rec(root.left, min_parent, max_parent)
        if root.right != None:
            r_max = rec(root.right, min_parent, max_parent)
    return max(l_max, r_max, self_max)


class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        return rec(root, root.val, root.val)
