from collections import namedtuple, defaultdict

MatchResult = namedtuple('MatchResult', ('winning_team', 'losing_team'))

class Match:
    def __init__(self, winning_team: str, losing_team: str):
        self.winning_team = winning_team
        self.losing_team = losing_team

    def __repr__(self):
        return f"{{{self.winning_team} : {self.losing_team}}}\n"

results = [Match('kings', 'sharks'),
           Match('penguins', 'flyers'),
           Match('rangers', 'sharks'),
           Match('rangers', 'penguins'),
           Match('sharks', 'penguins'),
           Match('flyers', 'sharks')]

def can_team_a_beat_team_b(matches, team_a, team_b):
    def build_graph():
        graph = defaultdict(set)
        for match in matches:
            graph[match.winning_team].add(match.losing_team)
        return graph

    def is_reachable_dfs(graph: defaultdict, curr, dest, visited=set()) -> bool:
        if curr == dest:
            return True
        elif curr in visited:
            return False
        elif curr not in graph:
            return False
        visited.add(curr)
        return any(is_reachable_dfs(graph, team, dest) for team in graph[curr])

    return is_reachable_dfs(build_graph(), team_a, team_b)

print(can_team_a_beat_team_b(results, 'rangers', 'kings'))
