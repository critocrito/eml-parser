<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![GPL-3.0 License][license-shield]][license-url]


<br />
<div align="center">
  <h3 align="center">EML Parser</h3>

  <p align="center">
    Analyze and extract data from a corpus of emails.
    <br />
    <a href="https://github.com/critocrito/eml-parser/issues">Report Bug</a>
    ·
    <a href="https://github.com/critocrito/eml-parser/issues">Request Feature</a>
  </p>
</div>


<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>


## About The Project

This code was developed to support investigations that took place at [Der SPIEGEL](https://www.spiegel.de) and [Paper Trail Media](https://www.papertrailmedia.de). 

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Getting Started

### Prerequisites

This project requires the [Rust](https://www.rust-lang.org) toolchain. The easiest way to install it is to use [Rustup](https://rustup.rs).

* 
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/critocrito/eml-parser.git
   ```
2. Compile the code.
   ```sh
   cargo build --release
   ```
3. Run the program.
   ```sh
   ./target/release/eml-parser --help
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Usage

This program reads all `.eml` files in a directory and sub-directories and extract or transforma data in it.

### List addresses and names

This command lists all email addresses and names in a corpus and count the occurences of this email/name appearing in the `From`, `To`, `Cc` or `Bcc` email header.

``` sh
eml-parser list -o people.csv mail-directory
```

### Turn a corpus to a network graph

The CSV file of the network graph can be used at [Cosmograph](https://cosmograph.app/run/) for visualization or further analyzed with tools like [graphctl](https://github.com/critocrito/graphctl).

``` sh
eml-parser network -o graph.csv mail-directory
```

### Extract all attachments

``` sh
eml-parser attachment -o attachments mail-directory
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## License

Distributed under the GPL-3.0 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Contact

Christo Buschek - [@christo_buschek](https://twitter.com/christo_buschek) - christo.buschek@proton.me

Project Link: [https://github.com/critocrito/eml-parser](https://github.com/critocrito/eml-parser)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


[contributors-shield]: https://img.shields.io/github/contributors/critocrito/eml-parser.svg?style=for-the-badge
[contributors-url]: https://github.com/critocrito/eml-parser/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/critocrito/eml-parser.svg?style=for-the-badge
[forks-url]: https://github.com/critocrito/eml-parser/network/members
[stars-shield]: https://img.shields.io/github/stars/critocrito/eml-parser.svg?style=for-the-badge
[stars-url]: https://github.com/critocrito/eml-parser/stargazers
[issues-shield]: https://img.shields.io/github/issues/critocrito/eml-parser.svg?style=for-the-badge
[issues-url]: https://github.com/critocrito/eml-parser/issues
[license-shield]: https://img.shields.io/github/license/critocrito/eml-parser.svg?style=for-the-badge
[license-url]: https://github.com/critocrito/eml-parser/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
