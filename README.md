# Substring Primes

Rust program to check for substring primes for first 1 million primes.

<u>__Substring prime__</u>: a prime where all substrings of that prime are also prime.

Example: "373" => 3 is prime, 7 is prime, 37 is prime, and 73 is prime, so:
- "373"
- "3__"
- "\_7\_"
- "__3"
- "37_"
- "_73"
- "373"

are all primes.

Interestingly enough, 373 is the largest possible substring prime. The complete list of substring primes is __2, 3, 5, 7, 23, 37, 53, 73, and 373__.

AI said I should add a run section so here you go.

## Running the Code
rustc >= 1.95.0

```bash
cargo run
```
No command line arguments; it simply goes through the first 1m primes and checks if they are substring primes. After it prints out the list of primes.

## An Aside
Also interesting is what 373 means in a broad variety of contexts. 373 refers to:
- __Angel Number 373__: a message from the angels that you have been doing a wonderful job in your daily life, and have been diligently and optimistically serving your life purpose and soul mission. (Angel numbers are especially interesting, as they seem to mesh numbers into a spiritual context. The phenomenon is called ___apophenia___, the human tendency to perceive meaningful connections, patterns, or intent within random or unrelated data. I prefer the term conspiracy theory).
- __373 Country Code__: for dialing codes, 373 is for Moldova. Moldova is also home to the worlds largest underground wine cellar. I'm jealous.
- __Sexy Prime__: 373 is 6 digits away from 367, making it a sexy prime. If you think 373 is sexy, consult your psychologist.
- __The Boiling Point of Water__: this is more of a stretch, but 373.15 Kelvin is the boiling point of water.
- __Sum of Prime Squares__: 373 = 3<sup>2</sup> + 5<sup>2</sup> + 7<sup>2</sup> + 11<sup>2</sup> + 13<sup>2</sup>
- __Fibonacci Exponents__: 373 = 2<sup>1</sup> + 3<sup>1</sup> + 5<sup>2</sup> + 7<sup>3</sup>, where the exponents are the first 4 digits of the fibonacci sequence.
- __Primitive Pythagorean Triple__: 373<sup>2</sup> = 252<sup>2</sup> + 275<sup>2</sup>
- __Logos Connection__: In Greek gematria (where letters are assigned numerical values), the word "Logos" (λόγος)—which translates to "The Word" in the Biblical context of John 1:1—adds up to exactly 373. (Greek gematria is how you get 666 from Nero; the Greek "Neron Kaisar: adds up to 1,005, but when the name is transliterated into the Hebrew letters nrwn qsr, the sum is 666, There's a fun little side tangent to go down on 666, but I'll leave that as an exercise to the reader).
- __Tarot Connection__: In numerology and tarot, 373 is often viewed visually. The number 3 represents "The Empress" (creativity and output), while 7 represents "The Chariot" (victory and alignment). The structure visually represents a season of victory safely framed by two eras of intense creativity.

Take from that what you will.
