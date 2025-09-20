import sys
import matplotlib.pyplot as plt
import numpy as np
import mplcursors


def sum_digits(n, base):
    s = 0
    while n > 0:
        s += n % base
        n //= base
    return s

def minimal_rotation(cycle):
    # Normalize cycle to minimal rotation (lexicographically smallest)
    if not cycle:
        return cycle
    n = len(cycle)
    rotations = [tuple(cycle[i:] + cycle[:i]) for i in range(n)]
    return min(rotations)

def detect_cycle(n, base, max_iter=100):
    history = []
    x = n
    for _ in range(max_iter):
        s = sum_digits(x, base)
        x = s * s
        if x in history:
            idx = history.index(x)
            cycle = history[idx:]
            return minimal_rotation(cycle)
        history.append(x)
    return None

def get_label(idx):
    # Returns a lowercase label: a-z, then aa, ab, ...
    import string
    letters = string.ascii_lowercase
    if idx < 26:
        return letters[idx]
    else:
        first = letters[(idx // 26) - 1]
        second = letters[idx % 26]
        return first + second

def to_base_digits(n, base):
    if n == 0:
        return [0]
    digits = []
    while n > 0:
        digits.append(n % base)
        n //= base
    return digits[::-1]


def main():
    if len(sys.argv) < 2:
        print("Usage: python hancock_pattern.py <base> [N] [vertical_spacing]")
        sys.exit(1)
    base = int(sys.argv[1])
    N = int(sys.argv[2]) if len(sys.argv) > 2 else 255
    vertical_spacing = int(sys.argv[3]) if len(sys.argv) > 3 else 20
    patterns = []
    cycle_to_label = {}
    label_to_cycle = {}
    label_idx = 0

    for n in range(1, N+1):
        cycle = detect_cycle(n, base)
        if cycle is None:
            label = 'other'
        else:
            if cycle not in cycle_to_label:
                lbl = get_label(label_idx)
                cycle_to_label[cycle] = lbl
                label_to_cycle[lbl] = cycle
                label_idx += 1
            label = cycle_to_label[cycle]
        patterns.append(label)

    # Map labels to y-values and colors
    unique_labels = sorted(set(patterns), key=lambda x: (x != 'other', x))
    label_to_y = {lbl: i * vertical_spacing for i, lbl in enumerate(unique_labels)}
    color_map = {lbl: plt.cm.tab20(i % 20) for i, lbl in enumerate(unique_labels)}

    x_vals = list(range(1, N+1))
    y_vals = [label_to_y[p] for p in patterns]
    colors = [color_map[p] for p in patterns]

    # Calculate figure height based on number of patterns and spacing
    fig_height = max(2, int(len(unique_labels) * vertical_spacing / 100))
    fig, ax = plt.subplots(figsize=(12, fig_height))
    scatter = ax.scatter(x_vals, y_vals, c=colors, s=10)
    ax.set_yticks([label_to_y[lbl] for lbl in unique_labels])
    ax.set_yticklabels(list(unique_labels), fontsize=8)
    ax.set_xlabel('Positive Integer')
    ax.set_ylabel('Pattern')
    ax.set_title(f'Hancock Pattern Scatter Plot (base {base}, N={N}, spacing={vertical_spacing})')

    # Add interactive tooltips for positive integer
    cursor = mplcursors.cursor(scatter, hover=True)
    @cursor.connect("add")
    def on_add(sel):
        idx = sel.index
        sel.annotation.set_text(f"n={x_vals[idx]}")

    plt.tight_layout()

    # Print key mapping pattern labels to cycle details
    print("\nPattern Key:")
    for lbl in sorted(label_to_cycle.keys()):
        cycle_str = ", ".join(".".join(str(d) for d in to_base_digits(x, base)) for x in label_to_cycle[lbl])
        print(f"  {lbl}: {cycle_str}")

    # Add legend to chart
    from matplotlib.patches import Patch
    legend_elements = [Patch(facecolor=color_map[lbl], label=lbl) for lbl in unique_labels]
    ax.legend(handles=legend_elements, title='Pattern', loc='upper right', bbox_to_anchor=(1, 1), fontsize=8)

    plt.show()

if __name__ == "__main__":
    main()
