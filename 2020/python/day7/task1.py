import string
from os.path import dirname


def count_containers():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('.\n')
        searched_bag = 'shiny gold'
        bags = {}
        for line in lines[:-1]:
            color, content = line.split(' bags contain ')
            if content == 'no other bags':
                continue
            inner_bag_colors = [(lambda bag: bag.strip('bags').strip(string.digits).strip())(content) for content in
                                content.split(', ')]
            bags[color] = inner_bag_colors
        possible_bags = find_containers(searched_bag, bags)
        return len(possible_bags)


def find_containers(searched_bag, bags):
    containers = set()
    for (color, content) in bags.items():
        if searched_bag in content:
            containers.add(color)
            containers |= find_containers(color, bags)
    return containers


if __name__ == '__main__':
    print(count_containers())
