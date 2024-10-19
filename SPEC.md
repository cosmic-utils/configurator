## Schema location

The schema must be stored in one of this locations

- `$XDG_DATA_HOME/configurator/`
- `$XDG_DATA_DIRS/configurator/`

The filename should be the [Application ID](https://docs.flathub.org/docs/for-app-authors/requirements/#application-id) of the application.

## Additional metadata

_note: list are separated by `;`._

<table>
  <thead>
    <tr>
      <th>Variable</th>
      <th>Description</th>
      <th>Default</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>X_CONFIGURATOR_SOURCE_PATHS</code></td>
      <td>Where the configuration will be sourced; in order.</td>
      <td></td>
      <td>List of paths</td>
    </tr>
    <tr>
      <td><code>X_CONFIGURATOR_SOURCE_HOME_PATHS</code></td>
      <td>Where the configuration will be sourced; in order, relative to <code>$HOME/</code></td>
      <td></td>
      <td>List of paths</td>
    </tr>
    <tr>
      <td><code>X_CONFIGURATOR_WRITE_PATH</code></td>
      <td>Where the configuration will be written.</td>
      <td>Last path from <code>X_CONFIGURATOR_SOURCE_PATHS</code></td>
      <td>Path</td>
    </tr>
    <tr>
      <td><code>X_CONFIGURATOR_FORMAT</code></td>
      <td>Format of the configuration. For COSMIC, it will be <code>ron_cosmic</code>.</td>
      <td>Extension of the last path in <code>X_CONFIGURATOR_SOURCE_HOME_PATHS</code></td>
      <td>String</td>
    </tr>
  </tbody>
</table>