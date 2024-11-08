from transformers import GPT2Tokenizer

#pipx run transformers python save_tokenizer.py
def save_tokenizer():
    # Load the GPT-2 tokenizer
    tokenizer = GPT2Tokenizer.from_pretrained("gpt2")
    # Save the tokenizer configuration to the specified path
    tokenizer.save_pretrained("./")

if __name__ == "__main__":
    save_tokenizer()