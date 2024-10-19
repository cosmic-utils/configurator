## Schema location

The schema must be stored in one of this locations:

- `$XDG_DATA_HOME/configurator/`
- `$XDG_DATA_DIRS/configurator/`

## Additional metadata

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
      <td>Required</td>
      <td>List of paths, separated by <code>;</code>. File or directory</td>
    </tr>
    <tr>
      <td><code>X_CONFIGURATOR_PATH</code></td>
      <td>Where the configuration will be written.</td>
      <td>Last path from <code>X_CONFIGURATOR_SOURCE_PATHS</code></td>
      <td>Path. File or directory</td>
    </tr>
    <tr>
      <td><code>X_CONFIGURATOR_FORMAT</code></td>
      <td>Format of the configuration. For COSMIC, it will be <code>ron_cosmic</code>.</td>
      <td>Extension of the last path in <code>X_CONFIGURATOR_SOURCE_PATHS</code></td>
      <td>String</td>
    </tr>
  </tbody>
</table>
