import { useState, useEffect, useRef } from 'react';
import { gpt2_backend } from 'declarations/gpt2_backend';
import { encode, decode } from 'gpt-tokenizer/model/davinci-002';

function App() {
  const [text, setText] = useState('');
  const [suggestion, setSuggestion] = useState('');
  const [tokenLength, setTokenLength] = useState(10); // Default token length
  const typingTimeoutRef = useRef(null);
  const fallbackTimeoutRef = useRef(null);

  useEffect(() => {
    return () => {
      if (typingTimeoutRef.current) {
        clearTimeout(typingTimeoutRef.current);
      }
      if (fallbackTimeoutRef.current) {
        clearTimeout(fallbackTimeoutRef.current);
      }
    };
  }, []);

  function handleChange(event) {
    setText(event.target.value);
    setSuggestion(''); // Clear suggestion when user starts typing

    if (typingTimeoutRef.current) {
      clearTimeout(typingTimeoutRef.current);
    }
  }

  async function handleSubmit() {
    if (!text.trim()) return;
    console.log('Submitting to backend...');

    const extractLastWords = (text) => {
      const words = text.split(' ');
      return words.slice(-10).join(' ');
    };

    const lastWords = extractLastWords(text);
    const tokensIds = encode(lastWords);

    // Start a fallback timer for 3 seconds, only when "Go" button is clicked
    fallbackTimeoutRef.current = setTimeout(() => {
      setSuggestion(
        "Error"
      );
      console.log('Fallback suggestion shown');
    }, 30000);

    try {
      // Call the backend and wait for the response, using user-specified token length
      const result = await gpt2_backend.model_inference(tokenLength, tokensIds);
      console.log('Backend result:', result);

      if (fallbackTimeoutRef.current) {
        clearTimeout(fallbackTimeoutRef.current); // Clear fallback if backend responds
      }

      if (result && result.Ok) {
        const decodedText = decode(result.Ok);
        console.log('Decoded suggestion:', decodedText);
        setSuggestion(decodedText);
      } else {
        console.error('Error: Backend did not return expected result.');
      }
    } catch (error) {
      console.error('Error fetching suggestion:', error);
    }
  }

  return (
    <main>
      <textarea
        id="text"
        value={text}
        onChange={handleChange}
        placeholder=""
        style={{ width: '100%', height: '200px' }}
      />
      <section id="suggestion">{suggestion}</section>

      <div>
        <label>
          Token Length:
          <input
            type="number"
            value={tokenLength}
            onChange={(e) => setTokenLength(Number(e.target.value))}
            style={{ margin: '10px', width: '50px' }}
          />
        </label>
        <button
          onClick={handleSubmit}
          style={{ marginLeft: '10px' }}
        >
          Go
        </button>
      </div>

      {suggestion && (
        <button
          onClick={() => {
            setText(text + ' ' + suggestion);
            setSuggestion('');
          }}
        >
          Insert
        </button>
      )}
    </main>
  );
}

export default App;
