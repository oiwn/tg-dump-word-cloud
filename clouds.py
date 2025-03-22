#!/usr/bin/env python
"""
Telegram Chat Word Cloud Generator
"""

import os
import sys
import matplotlib.pyplot as plt
from wordcloud import WordCloud
import argparse
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(description='Generate word cloud from preprocessed data')
    parser.add_argument('--input', '-i', required=True, help='Input file with word counts')
    parser.add_argument('--output', '-o', required=True, help='Output file for word cloud image')
    parser.add_argument('--width', type=int, default=800, help='Width of the word cloud image')
    parser.add_argument('--height', type=int, default=600, help='Height of the word cloud image')
    parser.add_argument('--font', default='DejaVu Sans', help='Font to use (must support Cyrillic)')
    parser.add_argument('--max-font-size', type=int, default=100, help='Maximum font size')
    parser.add_argument('--background', default='white', help='Background color')
    args = parser.parse_args()
    
    # Read the preprocessed word counts
    word_dict = {}
    with open(args.input, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split(' ')
            if len(parts) >= 2:
                word = ' '.join(parts[:-1])  # Handle multi-word tokens if any
                count = int(parts[-1])
                word_dict[word] = count
    
    print(f"Loaded {len(word_dict)} words")
    
    # Generate the word cloud
    wordcloud = WordCloud(
        width=args.width,
        height=args.height,
        max_font_size=args.max_font_size,
        background_color=args.background,
        font_path=args.font if os.path.exists(args.font) else None,
        font_step=1,
        regexp=r"\S+",  # Match any non-whitespace characters
        collocations=False,  # Don't include bigrams
        prefer_horizontal=0.9,  # Allow some vertical words
    ).generate_from_frequencies(word_dict)
    
    # Save the image
    plt.figure(figsize=(args.width / 100, args.height / 100), dpi=100)
    plt.imshow(wordcloud, interpolation='bilinear')
    plt.axis("off")
    plt.tight_layout(pad=0)
    plt.savefig(args.output, dpi=100)
    print(f"Word cloud saved to {args.output}")
    
    # Optional: show the image
    # plt.show()

if __name__ == "__main__":
    main()
