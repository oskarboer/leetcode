from collections import deque

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        parent = dict()
        infection = deque()
        
        q = deque()
        q.append(root)
        while len(q) > 0 :
            curr = q.pop()
            if curr.left != None:
                q.append(curr.left)
                parent[curr.left] = curr
            if curr.right != None:
                q.append(curr.right)
                parent[curr.right] = curr
            if curr.val == start:
                infection.append(curr)
                curr.val = 0
        
        ret = 0
        while len(infection) > 0:
            curr = infection.pop()
            if curr.left != None and curr.left.val > 0:
                infection.append(curr.left)
                curr.left.val = curr.val - 1
                ret = max(ret, abs(curr.left.val))
            if curr.right != None and curr.right.val > 0:
                infection.append(curr.right)
                curr.right.val = curr.val - 1
                ret = max(ret, abs(curr.right.val))
            p = parent.get(curr, None)
            if p != None and p.val > 0:
                infection.append(p)
                p.val = curr.val - 1
                ret = max(ret, abs(p.val))
        
        return ret