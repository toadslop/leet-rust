package graph.breadth_first.level_traversal;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;

class Solution2 {
  public List<List<Integer>> levelOrder(Node root) {
    List<List<Integer>> levelOrder = new ArrayList<>();
    if (root == null) {
      return levelOrder;
    }

    Map<Integer, Node> rights = new HashMap<>();
    LinkedList<Node> queue = new LinkedList<>();
    queue.add(root);

    List<Integer> workingList = new ArrayList<>();
    while (!queue.isEmpty()) {
      Node current = queue.poll();


    }

    return levelOrder;
  }
}
