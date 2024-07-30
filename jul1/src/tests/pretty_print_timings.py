import textwrap
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from collections import Counter
import string
from prettytable import PrettyTable
import tiktoken

# Load the CSV file
file_path = 'timings.csv'  # Adjust the path as needed
data = pd.read_csv(file_path)

# Get the tokenizer
tokenizer = tiktoken.encoding_for_model("gpt-4o")

# Function to count tokens
def count_tokens(text):
    return len(tokenizer.encode(text))

# Add token count to the dataframe
data['token_count'] = data['text'].apply(count_tokens)

# Distribution of timings
def plot_timing_distribution(data):
    sns.set(style="whitegrid")
    plt.figure(figsize=(12, 6))
    sns.histplot(data['duration'], kde=True, bins=30, color='skyblue', alpha=0.5)
    plt.title('Distribution of Timings')
    plt.xlabel('Duration')
    plt.ylabel('Frequency')
    plt.savefig('timing_distribution.png')
    plt.close()

    # Timings per character
    plt.figure(figsize=(12, 6))
    time_per_char = [row['duration'] / len(row['text']) * 1000 for _, row in data.iterrows()]
    sns.histplot(time_per_char, kde=True, bins=30, color='skyblue')
    plt.title('Distribution of Timings per Character')
    plt.xlabel('Time per Character (ms)')
    plt.ylabel('Frequency')
    plt.savefig('timing_per_char.png')
    plt.close()

    # Characters per second
    plt.figure(figsize=(12, 6))
    chars_per_second = [len(row['text']) / row['duration'] for _, row in data.iterrows()]
    sns.histplot(chars_per_second, kde=True, bins=30, color='lightgreen')
    plt.title('Distribution of Characters per Second')
    plt.xlabel('Characters per Second')
    plt.ylabel('Frequency')
    plt.savefig('chars_per_second.png')
    plt.close()

    # Tokens per second
    plt.figure(figsize=(12, 6))
    tokens_per_second = [row['token_count'] / row['duration'] for _, row in data.iterrows()]
    sns.histplot(tokens_per_second, kde=True, bins=30, color='salmon')
    plt.title('Distribution of Tokens per Second')
    plt.xlabel('Tokens per Second')
    plt.ylabel('Frequency')
    plt.savefig('tokens_per_second.png')
    plt.close()

    # Dunno why these are unreadable. The bars turn out white.
    # # Bar plot for characters per second over time
    # plt.figure(figsize=(12, 6))
    # plt.bar(data.index, chars_per_second, color='blue')
    # plt.title('Characters per Second Over Time')
    # plt.xlabel('Line Index')
    # plt.ylabel('Characters per Second')
    # plt.savefig('chars_per_second_over_time.png')
    # plt.close()
    #
    # # Bar plot for tokens per second over time
    # plt.figure(figsize=(12, 6))
    # plt.bar(data.index, tokens_per_second, color='red')
    # plt.title('Tokens per Second Over Time')
    # plt.xlabel('Line Index')
    # plt.ylabel('Tokens per Second')
    # plt.savefig('tokens_per_second_over_time.png')
    # plt.close()

# Pretty print the data
def pretty_print_data(data):
    table = PrettyTable()
    table.field_names = ["Index", "Char Count", "Token Count", "Duration", "Time per Char (ms)", "Time per Token (ms)", "Text"]
    table.align["Text"] = "l"
    for index, row in data.iterrows():
        text = row['text'][:100]
        duration = row['duration']
        char_count = len(text)
        token_count = row['token_count']
        time_per_char = duration / char_count * 1000
        time_per_token = duration / token_count * 1000
        table.add_row([index, char_count, token_count, duration, time_per_char, time_per_token, text])
    print(table)

# Plot the distributions
plot_timing_distribution(data)
pretty_print_data(data)