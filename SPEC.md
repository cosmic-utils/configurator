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
            <th>Required</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>X_CONFIG_CONFIGURATOR_SOURCE_PATHS</code></td>
            <td>Path to where the configuration will be sourced. The order matters. It can be a directory or a filename, depending on the format implementation.</td>
            <td>Yes</td>
        </tr>
        <tr>
            <td><code>X_CONFIG_CONFIGURATOR_PATH</code></td>
            <td>Path where the configuration will be written. It can be a directory or a filename, depending on the format implementation. Defaults to the last path from <code>X_CONFIG_CONFIGURATOR_SOURCE_PATHS</code>.</td>
            <td>No</td>
        </tr>
        <tr>
            <td><code>X_CONFIG_CONFIGURATOR_FORMAT</code></td>
            <td>The format of the configuration. By default, it uses the extension of the last path in <code>X_CONFIG_CONFIGURATOR_SOURCE_PATHS</code>. For COSMIC, it will be <code>ron_cosmic</code>.</td>
            <td>No</td>
        </tr>
    </tbody>
</table>
