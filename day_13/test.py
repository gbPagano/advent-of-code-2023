def find_mirror(grid):
    for r in range(1, len(grid)):
        above = grid[:r][::-1]
        below = grid[r:]
        
        if sum(sum(0 if a == b else 1 for a, b in zip(x, y)) for x, y in zip(above, below)) == 1:
            return r

    return 0

total = 0

for i, block in enumerate(open("input.txt").read().split("\n\n")):
    print(i)
    grid = block.splitlines()

    row = find_mirror(grid)
    if row:
        print(f"horizontal with {row} above")
    total += row * 100

    col = find_mirror(list(zip(*grid)))
    if col:
        print(f"vertical with {col} left")
    total += col

print(total)