<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Issues][issues-shield]][issues-url]
[![OSL-3.0 License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Sven65/TagBot">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">TagBot</h3>

  <p align="center">
    A tagbot for Discord.
    <br />
    <a href="https://github.com/Sven65/TagBot"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/Sven65/TagBot/issues">Report Bug</a>
    ·
    <a href="https://github.com/Sven65/TagBot/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
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
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![TagBot Screen Shot][product-screenshot]](https://github.com/Sven65/TagBot)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* cargo
  ```sh
  cargo run
  ```

* [rethinkdb](https://rethinkdb.com/)
	```sh
		docker run --name some-rethink -v "$PWD:/data" -d rethinkdb
	```

### Installation

1. Create an application at [https://discord.com/developers](https://discord.com/developers)
2. Clone the repo
   ```sh
   git clone https://github.com/Sven65/TagBot.git
   ```
3. Enter your bot token and RethinkDB credentials in `.env`
   ```env
   BOT_TOKEN=my_bot_token

   RETHINK_HOST=rethink_host
   ...
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

This bot has some commands that can be used in discord, the most prominent being `/tag`. For more, check the [`commands`](https://github.com/Sven65/TagBot/tree/master/src/commands/commands) directory.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] Legacy tag support
- [ ] Lua scripting
    - [ ] Serenity userdata
	- [ ] Serenity userdata functions

See the [open issues](https://github.com/Sven65/TagBot/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
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



<!-- LICENSE -->
## License

Distributed under the OSL-3.0 License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/Sven65/TagBot](https://github.com/Sven65/TagBot)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Best Readme Template](https://github.com/othneildrew/Best-README-Template)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[issues-shield]: https://img.shields.io/github/issues/Sven65/TagBot.svg?style=for-the-badge
[issues-url]: https://github.com/Sven65/TagBot/issues
[license-shield]: https://img.shields.io/github/license/Sven65/TagBot.svg?style=for-the-badge
[license-url]: https://github.com/Sven65/TagBot/blob/master/LICENSE
[product-screenshot]: images/screenshot.png