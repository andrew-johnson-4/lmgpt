lmgpt is an adapter to connect [LM](https://github.com/andrew-johnson-4/-) programs with the ChatGPT service.

### LM Example

```
history := λyyyy mm dd. Tell me a historical event that happened on (month mm) dd yyyy

error (history 1932 09 17)
```

### Running the Example

```
lmgpt example.lm
```

### Example Output

On September 17, 1932, the military coup known as the "Revolution of 1932" took place in Brazil.
This event marked a significant moment in Brazilian history and had lasting effects on the country's political landscape.

### Installation

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install LMGPT
3. Set your API key as an environment variable

```
cargo install lmgpt

export OPENAI_API_KEY=<your_api_key>
```
