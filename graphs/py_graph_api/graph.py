from typing import Iterable
from collections import deque
from collections import defaultdict
from collections import namedtuple


Edge = namedtuple('Edge', ('first', 'second'))
Vertex = object

class Graph:

    def __init__(self, pairs: Iterable) -> None:
        self.graph = defaultdict(set)
        self.edge_to = defaultdict(set)
        self.build_graph(pairs)

    def build_graph(self, pairs: Iterable) -> None:
        for pair in pairs:
            self.graph[pair.first].add(pair.second)

class GraphSearch:

    def __init__(self, src: Vertex) -> None:
        self._source = src
        self._marked = dict()
        self._edge_to = defaultdict(set)
        self._queue = deque()

    def has_path(self, v: Vertex) -> bool:
        return self._marked[v]

    def path_to(self, v: Vertex) -> list:
        if not self.has_path(v):
            return []

        path = []
        while v != self._source:
            path.append(v)
            v = self._edge_to[v]
        path.append(self._source)
        return list(reversed(path))


class DepthFirstPaths(GraphSearch):

    def __init__(self, graph: Graph, src: Vertex) -> None:
        super().__init__(src)
        self._dfs(graph, src)

    def _dfs(self, graph: Graph, src: Vertex):
        self._marked[src] = True

        for w in graph.graph[src]:
            if not self._marked.get(w):
                self._edge_to[w] = src
                self._dfs(graph, w)


class BreadthFirstPaths(GraphSearch):

    def __init__(self, graph: Graph, src: Vertex):
        super().__init__(src)
        self.bfs(graph, src)

    def bfs(self, graph: Graph, s: Vertex):
        self._queue = deque()
        self._queue.append(s)
        self._marked[s] = True

        while self._queue:
            v = self._queue.popleft()
            for w in graph.graph[v]:
                if not self._marked.get(w):
                    self._queue.append(w)
                    self._marked[w] = True
                    self._edge_to[w] = v
