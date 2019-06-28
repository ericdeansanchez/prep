import unittest
from graph import BreadthFirstPaths, Edge, Graph

class TestGraphBFS(unittest.TestCase):

    def test_basic_bfs(self):
        """
        Graph

        [1]-----[2]
                /|   [5]
               / |  /
              /  | /
           [4]--[3]

        :return:
        """
        edges = [
            Edge('one', 'two'),
            Edge('two', 'four'),
            Edge('two', 'three'),
            Edge('three', 'two'),
            Edge('three', 'four'),
            Edge('three', 'five'),
            Edge('four', 'two'),
            Edge('four', 'three'),
            Edge('five', 'three')
        ]

        g = Graph(edges)
        bfs_graph = BreadthFirstPaths(g, 'one')

        expected = ['one', 'two', 'three', 'five']
        actual = bfs_graph.path_to('five')
        self.assertEqual(expected, actual)


        expected = ['one', 'two', 'four']
        actual = bfs_graph.path_to('four')
        self.assertEqual(expected, actual)

        expected = ['one', 'two', 'three']
        actual = bfs_graph.path_to('three')
        self.assertEqual(expected, actual)

    def test_basic_bfs_alternate(self):
        """
        Graph

        [1]-----[2]
                /|   [5]
               / |  /
              /  | /
           [4]--[3]

        :return:
        """
        edges = [
            Edge('one', 'two'),
            Edge('two', 'four'),
            Edge('two', 'three'),
            Edge('three', 'two'),
            Edge('three', 'four'),
            Edge('three', 'five'),
            Edge('four', 'two'),
            Edge('four', 'three'),
            Edge('five', 'three')
        ]

        g = Graph(edges)
        bfs_graph = BreadthFirstPaths(g, 'five')

        expected = ['five', 'three', 'four']
        actual = bfs_graph.path_to('four')
        self.assertEqual(expected, actual)
        
