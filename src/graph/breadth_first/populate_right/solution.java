package graph.breadth_first.populate_right;

import java.util.LinkedList;

class Solution {
  public Node connect(Node root) {
    LinkedList<Node> queue = new LinkedList<>();
    queue.add(root);

    while (!queue.isEmpty()) {
      Node current = queue.poll();
      if (queue.peekFirst() != null) {
        Node next = queue.peekFirst();
        current.next = next;
      }

      if (current.left != null) {
        queue.add(current.left);
      }

      if (current.right != null) {
        queue.add(current.right);
      }
    }

    return root;
  }
}
