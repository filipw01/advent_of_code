from os.path import dirname


def count_declarations():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        declaration_batches = file.read().split('\n\n')
        total = 0
        for declaration_batch in declaration_batches:
            declared = set()
            for declaration in declaration_batch.replace('\n', ''):
                declared.add(declaration)
            total += len(declared)
        return total


if __name__ == '__main__':
    print(count_declarations())
