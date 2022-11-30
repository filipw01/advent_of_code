from os.path import dirname


def count_bags():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('.\n')
        searched_bag = 'shiny gold'
        bags = {}
        for line in lines[:-1]:
            color, content = line.split(' bags contain ')
            if content == 'no other bags':
                continue
            inner_bag_colors = [(lambda bag: bag.strip('bags').strip().split(' ', 1))(content) for content in
                                content.split(', ')]
            bags[color] = inner_bag_colors
        possible_bags = count_inner_bags(searched_bag, bags)
        return possible_bags


def count_inner_bags(searched_bag, bags, multiplier=1):
    count = 0
    try:
        for (inner_count, inner_color) in bags[searched_bag]:
            inner_count = int(inner_count)
            count += (inner_count + count_inner_bags(inner_color, bags, multiplier=inner_count)) * multiplier
        return count
    except KeyError:
        return count


if __name__ == '__main__':
    print(count_bags())
