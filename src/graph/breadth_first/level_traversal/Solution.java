package graph.breadth_first.level_traversal;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

class Solution {
  public List<List<Integer>> levelOrder(Node root) {
    List<List<Integer>> levelOrder = new ArrayList<>();
    if (root == null) {
      return levelOrder;
    }
    LinkedList<QueueItem> queue = new LinkedList<>();
    queue.add(new QueueItem(0, root));

    while (!queue.isEmpty()) {
      QueueItem currentItem = queue.poll();
      if (currentItem.getDetph() >= levelOrder.size()) {
        levelOrder.set(currentItem.getDetph(), new ArrayList<>());
      }
      levelOrder.get(currentItem.getDetph()).add(currentItem.getNode().val);

      for (Node child : currentItem.getNode().children) {
        queue.add(new QueueItem(currentItem.getDetph() + 1, child));
      }
    }

    return levelOrder;
  }
}
