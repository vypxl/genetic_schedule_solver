#! /usr/bin/env python
import csv
import sys

import matplotlib.pyplot as plt
import numpy as np


def normalize(means, bests, offset):
    return
    pivot = max(*means)

    for i in range(len(means)):
        if means[i] == -1:
            means[i] = float("nan")
        if bests[i] == -1:
            bests[i] = float("nan")
        means[i] /= pivot
        bests[i] /= pivot

        means[i] += offset
        bests[i] += offset


def plot_execution_data(file_path="all.csv"):
    data = np.genfromtxt(file_path, delimiter=",")
    data[data == -1] = np.nan

    plt.figure(figsize=(5, 5))

    plt.plot(data[:, 0], label="Small", linestyle="-", marker="", color="C2")
    plt.plot(data[:, 2], label="Medium", linestyle="-", marker="", color="C1")
    plt.plot(data[:, 4], label="Realistic", linestyle="-", marker="", color="C4")

    plt.xlabel("Generation")
    plt.ylabel("Penallty (given by eval. function)")
    plt.title("Best Timetables per Generation")
    plt.legend(loc=(0.7, 0.65))
    plt.grid(True)
    # log scale
    plt.yscale("log")
    plt.ylim(1, 10**5)
    # disable y scale
    # plt.gca().set_yticks([])
    # plt.gca().set_yticklabels([])
    plt.tight_layout()

    plt.savefig(file_path[:-4] + ".png")
    # plt.show()


def main():
    if len(sys.argv) != 2:
        print("Provide a file path.")
        exit(1)

    plot_execution_data(sys.argv[1])
    print(f"Generated {sys.argv[1][:-4]}.png")


if __name__ == "__main__":
    main()
