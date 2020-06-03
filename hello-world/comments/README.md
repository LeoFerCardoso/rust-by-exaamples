# Comments
 Any program requires comments, and Rust supports a few different varieties:

<ul>
    <li>Regular comments which are ignored by the compiler:
        <ul>
            <li>`// Line comments which go to the end of the line.`</li>
            <li>`/* Block comments which go to the closing delimiter. */`</li>
        </ul>
    </li>
    <li>Doc comments which are parsed into HTML library documentation:
        <ul>
            <li>`/// Generate library docs for the following item.`</li>
            <li>`//! Generate library docs for the enclosing item.`</li>
        </ul>
    </li>
</ul>