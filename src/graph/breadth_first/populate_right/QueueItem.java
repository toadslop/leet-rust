package graph.breadth_first.populate_right;

public class QueueItem {
  private Node node;
  private int depth;

  public QueueItem(Node _node, int _depth) {
    node = _node;
    depth = _depth;
  }

  public Node getNode() {
    return node;
  }

  public int getDepth() {
    return depth;
  }
}
