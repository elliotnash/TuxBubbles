tuxbubbles_sources = [
  'app/main.vala',
  'app/application.vala',
  'ui/window.vala',
  'ui/pages/conversation-list-page.vala',
  'ui/pages/conversation-view-page.vala',
  'ui/pages/onboarding.vala',
  'ui/widgets/message-composer.vala',
  'ui/widgets/conversation-row.vala',
  'domain/models/chat.vala',
  'services/settings.vala',
]

blueprints = custom_target('blueprints',
  input: files(
    'ui/shortcuts-dialog.blp',
    'ui/window.blp',
    'ui/pages/conversation-list-page.blp',
    'ui/pages/conversation-view-page.blp',
    'ui/pages/onboarding.blp',
    'ui/widgets/message-composer.blp',
    'ui/widgets/conversation-row.blp',
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'), 'batch-compile',
    meson.current_build_dir() / 'ui',              # output directory for generated .ui files
    meson.current_source_dir() / 'ui',             # base directory for sources
    '@INPUT@'
  ],
)

tuxbubbles_deps = [
  config_dep,
  dependency('gtk4'),
  dependency('libadwaita-1', version: '>= 1.8'),
  dependency('libsecret-1'),
]

ui_resources = gnome.compile_resources('ui-resources',
  'ui/ui_resources.gresource.xml',
  source_dir: [meson.current_build_dir() / 'ui', meson.current_source_dir() / 'ui'],
  dependencies: blueprints,
  c_name: 'ui_resources'
)

tuxbubbles_sources += ui_resources
tuxbubbles_sources += resources

executable('tuxbubbles', tuxbubbles_sources,
  dependencies: tuxbubbles_deps,
  include_directories: config_inc,
       install: true,
)
