from typing import List

class Node:
    def __init__(self, value: int):
        self.value: int = value
        self.next: 'Node' = None

    def __repr__(self):
        return f'({self.value}->{self.next.value})'

    def __eq__(self, other):
        return self.value == other


def play_game(values: List[int], steps: int):
    min_val = min(values)
    max_val = max(values)
    cups = [Node(v) for v in values]
    for i in range(len(cups) - 1):
        cups[i].next = cups[i + 1]
    cups[-1].next = cups[0]
    head = cups[0]
    lookup = {node.value: node for node in cups}
    for i in range(0, steps):
        grabbed = [head.next, head.next.next, head.next.next.next]
        head.next = grabbed[-1].next
        dest = head.value - 1
        if dest < min_val:
            dest = max_val
        while dest in grabbed:
            dest -= 1
            if dest < min_val:
                dest = max_val
        dest_node = lookup[dest]
        after = dest_node.next
        dest_node.next = grabbed[0]
        grabbed[-1].next = after
        head = head.next
    cup1 = lookup[1]
    print(cup1.next.value * cup1.next.next.value)


data = "364289715"
play_game([int(v) for v in data.strip()] + [v for v in range(len(data.strip())+1, 1000001)], 10000000)