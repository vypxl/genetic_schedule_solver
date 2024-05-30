#! /usr/bin/env python
import matplotlib.pyplot as plt
import csv
import sys

def normalize(means, bests, offset):
    pivot = max(*means)

    for i in range(len(means)):
        if means[i] == -1:
            means[i] = float('nan')
        if bests[i] == -1:
            bests[i] = float('nan')
        means[i] /= pivot
        bests[i] /= pivot

        means[i] += offset
        bests[i] += offset

def plot_execution_data(file_path='all.csv'):
    means1, bests1 = [], []
    means2, bests2 = [], []
    means3, bests3 = [], []

    with open(file_path, 'r') as file:
        reader = csv.reader(file)
        for row in reader:
            means1.append(float(row[0]))
            bests1.append(float(row[1]))
            means2.append(float(row[2]))
            bests2.append(float(row[3]))
            means3.append(float(row[4]))
            bests3.append(float(row[5]))

    normalize(means1, bests1, 0)
    normalize(means2, bests2, 1)
    normalize(means3, bests3, 2)

    plt.figure(figsize=(5, 5))

    # plt.plot(means1, label='Execution 1 Mean', linestyle='', marker='o', color='C2')
    plt.plot(bests1, label='Small', linestyle='', marker='o', color='C2')
    # plt.plot(means2, label='Execution 2 Mean', linestyle='', marker='o', color='C1')
    plt.plot(bests2, label='Medium', linestyle='', marker='o', color='C1')
    # plt.plot(means3, label='Execution 3 Mean', linestyle='', marker='o', color='C4')
    plt.plot(bests3, label='Realistic', linestyle='', marker='o', color='C4')

    plt.xlabel('Generation')
    plt.ylabel('Penallty (relative)')
    plt.title('Mean and Best Values of Executions')
    plt.legend(loc='best')
    plt.grid(True)
    plt.ylim(0, 4)
    # disable y scale
    plt.gca().set_yticks([])
    plt.gca().set_yticklabels([])
    plt.tight_layout()

    plt.savefig('chart.png')
    plt.show()

def main():
    if len(sys.argv) != 2:
        print("Provide a file path.")
        exit(1)

    plot_execution_data(sys.argv[1])

if __name__ == "__main__":
    main()
