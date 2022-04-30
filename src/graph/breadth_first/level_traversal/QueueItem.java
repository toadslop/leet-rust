package graph.breadth_first.level_traversal;

public class QueueItem {
  private Integer depth;
  private Node node;

  public QueueItem(Integer nodeDepth, Node currentNode) {
    depth = nodeDepth;
    node = currentNode;
  }

  Integer getDetph() {
    return depth;
  }

  Node getNode() {
    return node;
  }
}
