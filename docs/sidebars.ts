import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'index',
    {
      type: 'category',
      label: 'Introduction',
      collapsed: false,
      items: ['intro', 'installation', 'quick-start', 'why-forgelang', 'examples'],
    },
    {
      type: 'category',
      label: 'Language Guide',
      collapsed: false,
      items: [
        'language-guide/overview',
        'language-guide/syntax',
        'language-guide/types',
        'language-guide/variables',
        'language-guide/operators',
        'language-guide/control-flow',
        'language-guide/functions',
        'language-guide/modules',
      ],
    },
    {
      type: 'category',
      label: 'Advanced Features',
      collapsed: true,
      items: [
        'advanced/generics',
        'advanced/enums',
      ],
    },
    {
      type: 'category',
      label: 'Standard Library',
      collapsed: false,
      items: [
        'stdlib/overview',
        'stdlib/io',
        'stdlib/math',
        'stdlib/list',
        'stdlib/str',
        'stdlib/bool',
        'stdlib/int',
        'stdlib/float',
        'stdlib/fs',
        'stdlib/time',
        'stdlib/env',
      ],
    },
    {
      type: 'category',
      label: 'API Reference',
      collapsed: true,
      items: [
        'api/overview',
        'api/io',
      ],
    },
  ],
};

export default sidebars;
