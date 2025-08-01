# Style Element Position

Sometimes you want to control where the style element should be inserted.

You can create a <meta name="thaw-ui-style" /> element inside head element, then thaw-ui's style will be inserted right before it.

```html
<head>
  <xxx />
  <!-- thaw-ui's style will be inserted here -->
  <meta name="thaw-ui-style" />
  <xxx />
</head>
```
