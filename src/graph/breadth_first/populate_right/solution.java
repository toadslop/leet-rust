package graph.breadth_first.populate_right;

import java.util.LinkedList;

class Solution {
  public Node connect(Node root) {
    if (root == null) {
      return root;
    }
    LinkedList<QueueItem> queue = new LinkedList<>();
    queue.add(new QueueItem(root, 0));

    while (!queue.isEmpty()) {
      QueueItem currentItem = queue.poll();
      Node currentNode = currentItem.getNode();
      int currentDepth = currentItem.getDepth();

      QueueItem nextItem = queue.peekFirst();
      if (nextItem != null && nextItem.getDepth() == currentDepth) {
        currentNode.next = nextItem.getNode();
      }

      if (currentNode.left != null) {
        queue.add(new QueueItem(currentNode.next, currentDepth + 1));
      }

      if (currentNode.right != null) {
        queue.add(new QueueItem(currentNode.right, currentDepth + 1));
      }
    }

    return root;
  }

}
