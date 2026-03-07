// @ts-check
// ForgeLang Docusaurus Configuration
// Material 3-inspired theme with comprehensive documentation

import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'ForgeLang',
  tagline: 'A dynamically-typed interpreted language with focus on simplicity and expressiveness',
  favicon: 'img/favicon.ico',

  url: 'https://forgelang.dev',
  baseUrl: '/',
  organizationName: 'ForgeLang',
  projectName: 'forgelang',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/forgelang/forgelang/tree/main/docs/',
          routeBasePath: '/docs',
          path: 'docs',
          showLastUpdateAuthor: true,
          showLastUpdateTime: true,
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/og-image.png',
    navbar: {
      title: 'ForgeLang',
      logo: {
        alt: 'ForgeLang Logo',
        src: 'img/logo.svg',
        srcDark: 'img/logo-dark.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Documentation',
        },
        {
          to: '/docs/api/overview',
          label: 'API Reference',
          position: 'left',
        },
        {
          to: '/docs/examples',
          label: 'Examples',
          position: 'left',
        },
        {
          href: 'https://github.com/forgelang/forgelang',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            { label: 'Getting Started', to: '/docs/intro' },
            { label: 'Language Guide', to: '/docs/language-guide/overview' },
            { label: 'Standard Library', to: '/docs/stdlib/overview' },
          ],
        },
        {
          title: 'Community',
          items: [
            { label: 'GitHub', href: 'https://github.com/forgelang/forgelang' },
            { label: 'Discord', href: 'https://discord.gg/forgelang' },
            { label: 'Twitter', href: 'https://twitter.com/forgelang' },
          ],
        },
        {
          title: 'More',
          items: [
            { label: 'Blog', href: 'https://github.com/forgelang/forgelang' },
            { label: 'Showcase', href: 'https://github.com/forgelang/forgelang' },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} ForgeLang. Built with Docusaurus and Material Design.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'bash', 'json', 'yaml'],
    },
    colorMode: {
      defaultMode: 'light',
      disableSwitch: false,
      respectPrefersColorScheme: true,
    },
    docs: {
      sidebar: {
        hideable: true,
        autoCollapseCategories: true,
      },
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
