<p align="center">
    <svg xmlns="http://www.w3.org/2000/svg" width="240" height="240" viewBox="0 0 24 24" fill="#dea584"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/></svg>
    <br /><br />
    <img src="https://img.shields.io/badge/Made%20with-Rust-orange.svg">
    <a href="https://GitHub.com/tfSheol/lnkds/releases"><img src="https://img.shields.io/github/release/tfSheol/lnkds.svg"></a>
    <a href="https://github.com/tfSheol/lnkds/commits"><img alt="GitHub commits since latest release (by date)" src="https://img.shields.io/github/commits-since/tfSheol/lnkds/latest"></a>
    <p align="center">Scrapp all informations you whant of linkedIn profile's. 🚀</p>
    <p align="center">Work since 📅 <b>08/22/2021</b></p>
    <br />
</p>

# lnkds

With lnkds you can scrapp all profile's informations you whant with your `li_at` token provided by your linkedin cookie.

```bash
$ lnkds --help

lnkds 0.1.2

Teddy F. <pro@teddyfontaine.fr>

This doc string acts as a help message when the user runs '--help' as do all doc strings on fields

USAGE:
    lnkds [FLAGS] <LI_AT> <SUBCOMMAND>

ARGS:
    <LI_AT>    "li_at" from LinkedIn Cookie

FLAGS:
    -h, --help       Print help information
    -v, --verbose    A level of verbosity, and can be used multiple times
    -V, --version    Print version information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    profile    A subcommand to Get LinkedIn profile (basic informations, experiences,
               certificates, ...)
```

## lnkds-profile

Scrapp a full linkedin profile with basic informations, experiences, educations, certifications, skills, projects and organizations.

```bash
$ lnkds <AQE...> profile <tf...> --help

lnkds-profile

A subcommand to Get LinkedIn profile (basic informations, experiences, certificates, ...)

USAGE:
    lnkds <LI_AT> profile <USER>

ARGS:
    <USER>    User account you whant to parse

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Parsing rules

- Experiences: `"$type": "com.linkedin.voyager.dash.identity.profile.Position"`
- Experiences_groups: `"$type": "com.linkedin.voyager.dash.identity.profile.PositionGroup"`
- Medias: `"$type": "com.linkedin.voyager.dash.identity.profile.treasury.TreasuryMedia"`
- Profile: `"publicIdentifier": "teddyfontaine"` + `"$type": "com.linkedin.voyager.dash.identity.profile.Profile"`
- Educations: `"$type": "com.linkedin.voyager.dash.identity.profile.Education"`
- Certifications: `"$type": "com.linkedin.voyager.dash.identity.profile.Certification"`
- Skills: `"$type": "com.linkedin.voyager.dash.identity.profile.Skill"`
- Projects: `"$type": "com.linkedin.voyager.dash.identity.profile.Project"`
- Organizations: `"$type": "com.linkedin.voyager.dash.identity.profile.Organization"`

## Realase

This project support `@release-it` with 2 plugins `@release-it/keep-a-changelog` and `@release-it/bumper`.

- [@release-it](https://github.com/release-it/release-it)
- [@release-it/keep-a-changelog](https://github.com/release-it/keep-a-changelog)
- [@release-it/bumper](https://github.com/release-it/bumper)

```bash
$ sudo npm install -g release-it
$ sudo npm install -g @release-it/keep-a-changelog
$ sudo npm install -g 
```

## Voyager api limitations

- This api can't getted over 18 experiences.

## Todos

- [x] Implementation of user experiences
- [ ] Implementation of user experiences by groups
- [ ] Implementation of experiences medias
- [ ] Implementation of user profile
- [ ] Implementation of user educations
- [ ] Implementation of user certifications
- [ ] Implementation of user skills
- [ ] Implementation of user projects
- [ ] Implementation of user organizations