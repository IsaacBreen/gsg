import textwrap

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from collections import Counter
import string
from prettytable import PrettyTable

# Load the CSV file
file_path = 'timings.csv'  # Adjust the path as needed
data = pd.read_csv(file_path)


# Distribution of timings
def plot_timing_distribution(data):
    sns.set(style="whitegrid")
    plt.figure(figsize=(12, 6))
    sns.histplot(data['duration'], kde=True, bins=30, color='skyblue', alpha=0.5)
    plt.title('Distribution of Timings')
    plt.xlabel('Duration')
    plt.ylabel('Frequency')
    plt.savefig('timing_distribution.png')

    # Timings per character
    sns.set(style="whitegrid")
    plt.figure(figsize=(12, 6))
    time_per_char = [row['duration'] / len(row['text']) * 1000 for _, row in data.iterrows()]
    sns.histplot(time_per_char, kde=True, bins=30, color='skyblue')
    plt.title('Distribution of Timings per Character')
    plt.xlabel('Time per Character (ms)')
    plt.ylabel('Frequency')
    plt.savefig('timing_per_char.png')


# Pretty print the data
def pretty_print_data(data):
    table = PrettyTable()
    table.field_names = ["Index", "Character Count", "Duration", "Time per Character (ms)", "Text"]
    # Align the text column left and make sure it doesn't go over the page width. If it does, don't wrap it, cut it off, and put ellipses at the end.
    table.align["Text"] = "l"

    for index, row in data.iterrows():
        text = row['text'][:100]
        duration = row['duration']
        char_count = len(text)
        time_per_char = duration / char_count * 1000
        table.add_row([index, char_count, duration, time_per_char, text])

    print(table)


# Plot the distributions
plot_timing_distribution(data)

pretty_print_data(data)