
const html_src = `
/// The \`<a>\` HTML element (or anchor element), with its href attribute, creates a hyperlink to web pages, files, email addresses, locations in the same page, or anything else a URL can address.
    a HtmlAnchorElement [download, href, hreflang, ping, referrerpolicy, rel, target, r#type ] true,
    /// The \`<abbr>\` HTML element represents an abbreviation or acronym; the optional title attribute can provide an expansion or description for the abbreviation. If present, title must contain this full description and nothing else.
    abbr HtmlElement [] true,
    /// The \`<address>\` HTML element indicates that the enclosed HTML provides contact information for a person or people, or for an organization.
    address HtmlElement [] true,
    /// The \`<article>\` HTML element represents a self-contained composition in a document, page, application, or site, which is intended to be independently distributable or reusable (e.g., in syndication). Examples include: a forum post, a magazine or newspaper article, or a blog entry, a product card, a user-submitted comment, an interactive widget or gadget, or any other independent item of content.
    article HtmlElement [] true,
    /// The \`<aside>\` HTML element represents a portion of a document whose content is only indirectly related to the document's main content. Asides are frequently presented as sidebars or call-out boxes.
    aside HtmlElement [] true,
    /// The \`<audio>\` HTML element is used to embed sound content in documents. It may contain one or more audio sources, represented using the src attribute or the source element: the browser will choose the most suitable one. It can also be the destination for streamed media, using a MediaStream.
    audio HtmlAudioElement [autoplay, controls, crossorigin, r#loop, muted, preload, src] true,
    /// The \`<b>\` HTML element is used to draw the reader's attention to the element's contents, which are not otherwise granted special importance. This was formerly known as the Boldface element, and most browsers still draw the text in boldface. However, you should not use \`<b>\` for styling text; instead, you should use the CSS font-weight property to create boldface text, or the strong element to indicate that text is of special importance.
    b HtmlElement [] true,
    /// The \`<bdi>\` HTML element tells the browser's bidirectional algorithm to treat the text it contains in isolation from its surrounding text. It's particularly useful when a website dynamically inserts some text and doesn't know the directionality of the text being inserted.
    bdi HtmlElement [] true,
    /// The \`<bdo>\` HTML element overrides the current directionality of text, so that the text within is rendered in a different direction.
    bdo HtmlElement [] true,
    /// The \`<blockquote>\` HTML element indicates that the enclosed text is an extended quotation. Usually, this is rendered visually by indentation (see Notes for how to change it). A URL for the source of the quotation may be given using the cite attribute, while a text representation of the source can be given using the cite element.
    blockquote HtmlQuoteElement [cite] true,
    /// The \`<body>\` HTML element represents the content of an HTML document. There can be only one \`<body>\` element in a document.
    body HtmlBodyElement [] true,
    /// The \`<button>\` HTML element represents a clickable button, used to submit forms or anywhere in a document for accessible, standard button functionality.
    button HtmlButtonElement [command, commandfor, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, name, r#type, value, popovertarget, popovertargetaction] true,
    /// Use the HTML \`<canvas>\` element with either the canvas scripting API or the WebGL API to draw graphics and animations.
    canvas HtmlCanvasElement [height, width] true,
    /// The \`<caption>\` HTML element specifies the caption (or title) of a table.
    caption HtmlTableCaptionElement [] true,
    /// The \`<cite>\` HTML element is used to describe a reference to a cited creative work, and must include the title of that work. The reference may be in an abbreviated form according to context-appropriate conventions related to citation metadata.
    cite HtmlElement [] true,
    /// The \`<code>\` HTML element displays its contents styled in a fashion intended to indicate that the text is a short fragment of computer code. By default, the content text is displayed using the user agent default monospace font.
    code HtmlElement [] true,
    /// The \`<colgroup>\` HTML element defines a group of columns within a table.
    colgroup HtmlTableColElement [span] true,
    /// The \`<data>\` HTML element links a given piece of content with a machine-readable translation. If the content is time- or date-related, the time element must be used.
    data HtmlDataElement [value] true,
    /// The \`<datalist>\` HTML element contains a set of option elements that represent the permissible or recommended options available to choose from within other controls.
    datalist HtmlDataListElement [] true,
    /// The \`<dd>\` HTML element provides the description, definition, or value for the preceding term (dt) in a description list (dl).
    dd HtmlElement [] true,
    /// The \`<del>\` HTML element represents a range of text that has been deleted from a document. This can be used when rendering "track changes" or source code diff information, for example. The ins element can be used for the opposite purpose: to indicate text that has been added to the document.
    del HtmlModElement [cite, datetime] true,
    /// The \`<details>\` HTML element creates a disclosure widget in which information is visible only when the widget is toggled into an "open" state. A summary or label must be provided using the summary element.
    details HtmlDetailsElement [name, open] true,
    /// The \`<dfn>\` HTML element is used to indicate the term being defined within the context of a definition phrase or sentence. The p element, the dt/dd pairing, or the section element which is the nearest ancestor of the \`<dfn>\` is considered to be the definition of the term.
    dfn HtmlElement [] true,
    /// The \`<dialog>\` HTML element represents a dialog box or other interactive component, such as a dismissible alert, inspector, or subwindow.
    dialog HtmlDialogElement [closedby, open] true,
    /// The \`<div>\` HTML element is the generic container for flow content. It has no effect on the content or layout until styled in some way using CSS (e.g. styling is directly applied to it, or some kind of layout model like Flexbox is applied to its parent element).
    div HtmlDivElement [] true,
    /// The \`<dl>\` HTML element represents a description list. The element encloses a list of groups of terms (specified using the dt element) and descriptions (provided by dd elements). Common uses for this element are to implement a glossary or to display metadata (a list of key-value pairs).
    dl HtmlDListElement [] true,
    /// The \`<dt>\` HTML element specifies a term in a description or definition list, and as such must be used inside a dl element. It is usually followed by a dd element; however, multiple \`<dt>\` elements in a row indicate several terms that are all defined by the immediate next dd element.
    dt HtmlElement [] true,
    /// The \`<em>\` HTML element marks text that has stress emphasis. The \`<em>\` element can be nested, with each level of nesting indicating a greater degree of emphasis.
    em HtmlElement [] true,
    /// The \`<fieldset>\` HTML element is used to group several controls as well as labels (label) within a web form.
    fieldset HtmlFieldSetElement [disabled, form, name] true,
    /// The \`<figcaption>\` HTML element represents a caption or legend describing the rest of the contents of its parent figure element.
    figcaption HtmlElement [] true,
    /// The \`<figure>\` HTML element represents self-contained content, potentially with an optional caption, which is specified using the figcaption element. The figure, its caption, and its contents are referenced as a single unit.
    figure HtmlElement [] true,
    /// The \`<footer>\` HTML element represents a footer for its nearest sectioning content or sectioning root element. A \`<footer>\` typically contains information about the author of the section, copyright data or links to related documents.
    footer HtmlElement [] true,
    /// The \`<form>\` HTML element represents a document section containing interactive controls for submitting information.
    form HtmlFormElement [accept_charset, action, autocomplete, enctype, method, name, novalidate, target] true,
    /// The \`<h1>\` to \`<h6>\` HTML elements represent six levels of section headings. \`<h1>\` is the highest section level and \`<h6>\` is the lowest.
    h1 HtmlHeadingElement [] true,
    /// The \`<h1>\` to \`<h6>\` HTML elements represent six levels of section headings. \`<h1>\` is the highest section level and \`<h6>\` is the lowest.
    h2 HtmlHeadingElement [] true,
    /// The \`<h1>\` to \`<h6>\` HTML elements represent six levels of section headings. \`<h1>\` is the highest section level and \`<h6>\` is the lowest.
    h3 HtmlHeadingElement [] true,
    /// The \`<h1>\` to \`<h6>\` HTML elements represent six levels of section headings. \`<h1>\` is the highest section level and \`<h6>\` is the lowest.
    h4 HtmlHeadingElement [] true,
    /// The \`<h1>\` to \`<h6>\` HTML elements represent six levels of section headings. \`<h1>\` is the highest section level and \`<h6>\` is the lowest.
    h5 HtmlHeadingElement [] true,
    /// The \`<h1>\` to \`<h6>\` HTML elements represent six levels of section headings. \`<h1>\` is the highest section level and \`<h6>\` is the lowest.
    h6 HtmlHeadingElement [] true,
    ///	The \`<head>\` HTML element contains machine-readable information (metadata) about the document, like its title, scripts, and style sheets.
    head HtmlHeadElement [] true,
    /// The \`<header>\` HTML element represents introductory content, typically a group of introductory or navigational aids. It may contain some heading elements but also a logo, a search form, an author name, and other elements.
    header HtmlElement [] true,
    /// The \`<hgroup>\` HTML element represents a heading and related content. It groups a single \`<h1>–<h6>\` element with one or more \`<p>\`.
    hgroup HtmlElement [] true,
    /// The \`<html>\` HTML element represents the root (top-level element) of an HTML document, so it is also referred to as the root element. All other elements must be descendants of this element.
    html HtmlHtmlElement [] true,
    /// The \`<i>\` HTML element represents a range of text that is set off from the normal text for some reason, such as idiomatic text, technical terms, taxonomical designations, among others. Historically, these have been presented using italicized type, which is the original source of the \`<i>\` naming of this element.
    i HtmlElement [] true,
    /// The \`<iframe>\` HTML element represents a nested browsing context, embedding another HTML page into the current one.
    iframe HtmlIFrameElement [allow, allowfullscreen, allowpaymentrequest, height, name, referrerpolicy, sandbox, src, srcdoc, width] true,
    /// The \`<ins>\` HTML element represents a range of text that has been added to a document. You can use the del element to similarly represent a range of text that has been deleted from the document.
    ins HtmlElement [cite, datetime] true,
    /// The \`<kbd>\` HTML element represents a span of inline text denoting textual user input from a keyboard, voice input, or any other text entry device. By convention, the user agent defaults to rendering the contents of a \`<kbd>\` element using its default monospace font, although this is not mandated by the HTML standard.
    kbd HtmlElement [] true,
    /// The \`<label>\` HTML element represents a caption for an item in a user interface.
    label HtmlLabelElement [r#for, form] true,
    /// The \`<legend>\` HTML element represents a caption for the content of its parent fieldset.
    legend HtmlLegendElement [] true,
    /// The \`<li>\` HTML element is used to represent an item in a list. It must be contained in a parent element: an ordered list (ol), an unordered list (ul), or a menu (menu). In menus and unordered lists, list items are usually displayed using bullet points. In ordered lists, they are usually displayed with an ascending counter on the left, such as a number or letter.
    li HtmlLiElement [value] true,
    /// The \`<main>\` HTML element represents the dominant content of the body of a document. The main content area consists of content that is directly related to or expands upon the central topic of a document, or the central functionality of an application.
    main HtmlElement [] true,
    /// The \`<map>\` HTML element is used with area elements to define an image map (a clickable link area).
    map HtmlMapElement [name] true,
    /// The \`<mark>\` HTML element represents text which is marked or highlighted for reference or notation purposes, due to the marked passage's relevance or importance in the enclosing context.
    mark HtmlElement [] true,
    /// The \`<menu>\` HTML element is a semantic alternative to ul. It represents an unordered list of items (represented by li elements), each of these represent a link or other command that the user can activate.
    menu HtmlMenuElement [] true,
    /// The \`<meter>\` HTML element represents either a scalar value within a known range or a fractional value.
    meter HtmlMeterElement [value, min, max, low, high, optimum, form] true,
    /// The \`<nav>\` HTML element represents a section of a page whose purpose is to provide navigation links, either within the current document or to other documents. Common examples of navigation sections are menus, tables of contents, and indexes.
    nav HtmlElement [] true,
    /// The \`<noscript>\` HTML element defines a section of HTML to be inserted if a script type on the page is unsupported or if scripting is currently turned off in the browser.
    noscript HtmlElement [] false,
    /// The \`<object>\` HTML element represents an external resource, which can be treated as an image, a nested browsing context, or a resource to be handled by a plugin.
    object HtmlObjectElement [data, form, height, name, r#type, usemap, width] true,
    /// The \`<ol>\` HTML element represents an ordered list of items — typically rendered as a numbered list.
    ol HtmlOListElement [reversed, start, r#type] true,
    /// The \`<optgroup>\` HTML element creates a grouping of options within a select element.
    optgroup HtmlOptGroupElement [disabled, label] true,
    /// The \`<output>\` HTML element is a container element into which a site or app can inject the results of a calculation or the outcome of a user action.
    output HtmlOutputElement [r#for, form, name] true,
    /// The \`<p>\` HTML element represents a paragraph. Paragraphs are usually represented in visual media as blocks of text separated from adjacent blocks by blank lines and/or first-line indentation, but HTML paragraphs can be any structural grouping of related content, such as images or form fields.
    p HtmlParagraphElement [] true,
    /// The \`<picture>\` HTML element contains zero or more source elements and one img element to offer alternative versions of an image for different display/device scenarios.
    picture HtmlPictureElement [] true,
    /// The \`<portal>\` HTML element enables the embedding of another HTML page into the current one for the purposes of allowing smoother navigation into new pages.
    portal HtmlElement [referrerpolicy, src] true,
    /// The \`<pre>\` HTML element represents preformatted text which is to be presented exactly as written in the HTML file. The text is typically rendered using a non-proportional, or "monospaced, font. Whitespace inside this element is displayed as written.
    pre HtmlPreElement [] true,
    /// The \`<progress>\` HTML element displays an indicator showing the completion progress of a task, typically displayed as a progress bar.
    progress HtmlProgressElement [min, max, value] true,
    /// The \`<q>\` HTML element indicates that the enclosed text is a short inline quotation. Most modern browsers implement this by surrounding the text in quotation marks. This element is intended for short quotations that don't require paragraph breaks; for long quotations use the blockquote element.
    q HtmlQuoteElement [cite] true,
    /// The \`<rp>\` HTML element is used to provide fall-back parentheses for browsers that do not support display of ruby annotations using the ruby element. One \`<rp>\` element should enclose each of the opening and closing parentheses that wrap the rt element that contains the annotation's text.
    rp HtmlElement [] true,
    /// The \`<rt>\` HTML element specifies the ruby text component of a ruby annotation, which is used to provide pronunciation, translation, or transliteration information for East Asian typography. The \`<rt>\` element must always be contained within a ruby element.
    rt HtmlElement [] true,
    /// The \`<ruby>\` HTML element represents small annotations that are rendered above, below, or next to base text, usually used for showing the pronunciation of East Asian characters. It can also be used for annotating other kinds of text, but this usage is less common.
    ruby HtmlElement [] true,
    /// The \`<s>\` HTML element renders text with a strikethrough, or a line through it. Use the \`<s>\` element to represent things that are no longer relevant or no longer accurate. However, \`<s>\` is not appropriate when indicating document edits; for that, use the del and ins elements, as appropriate.
    s HtmlElement [] true,
    /// The \`<samp>\` HTML element is used to enclose inline text which represents sample (or quoted) output from a computer program. Its contents are typically rendered using the browser's default monospaced font (such as Courier or Lucida Console).
    samp HtmlElement [] true,
    /// The \`<script>\` HTML element is used to embed executable code or data; this is typically used to embed or refer to JavaScript code. The \`<script>\` element can also be used with other languages, such as WebGL's GLSL shader programming language and JSON.
    script HtmlScriptElement [r#async, crossorigin, defer, fetchpriority, integrity, nomodule, referrerpolicy, src, r#type, blocking] false,
    /// The \`<search>\` HTML element is a container representing the parts of the document or application with form controls or other content related to performing a search or filtering operation.
    search HtmlElement [] true,
    /// The \`<section>\` HTML element represents a generic standalone section of a document, which doesn't have a more specific semantic element to represent it. Sections should always have a heading, with very few exceptions.
    section HtmlElement [] true,
    /// The \`<select>\` HTML element represents a control that provides a menu of options:
    select HtmlSelectElement [autocomplete, disabled, form, multiple, name, required, size] true,
    /// The \`<slot>\` HTML element—part of the Web Components technology suite—is a placeholder inside a web component that you can fill with your own markup, which lets you create separate DOM trees and present them together.
    slot HtmlSlotElement [name] true,
    /// The \`<small>\` HTML element represents side-comments and small print, like copyright and legal text, independent of its styled presentation. By default, it renders text within it one font-size smaller, such as from small to x-small.
    small HtmlElement [] true,
    /// The \`<span>\` HTML element is a generic inline container for phrasing content, which does not inherently represent anything. It can be used to group elements for styling purposes (using the class or id attributes), or because they share attribute values, such as lang. It should be used only when no other semantic element is appropriate. \`<span>\` is very much like a div element, but div is a block-level element whereas a \`<span>\` is an inline element.
    span HtmlSpanElement [] true,
    /// The \`<strong>\` HTML element indicates that its contents have strong importance, seriousness, or urgency. Browsers typically render the contents in bold type.
    strong HtmlElement [] true,
    ///	The \`<style>\` HTML element contains style information for a document, or part of a document. It contains CSS, which is applied to the contents of the document containing the \`<style>\` element.
    style HtmlStyleElement [media, blocking] false,
    /// The \`<sub>\` HTML element specifies inline text which should be displayed as subscript for solely typographical reasons. Subscripts are typically rendered with a lowered baseline using smaller text.
    sub HtmlElement [] true,
    /// The \`<summary>\` HTML element specifies a summary, caption, or legend for a details element's disclosure box. Clicking the \`<summary>\` element toggles the state of the parent \`<details>\` element open and closed.
    summary HtmlElement [] true,
    /// The \`<sup>\` HTML element specifies inline text which is to be displayed as superscript for solely typographical reasons. Superscripts are usually rendered with a raised baseline using smaller text.
    sup HtmlElement [] true,
    /// The \`<table>\` HTML element represents tabular data — that is, information presented in a two-dimensional table comprised of rows and columns of cells containing data.
    table HtmlTableElement [] true,
    /// The \`<tbody>\` HTML element encapsulates a set of table rows (tr elements), indicating that they comprise the body of the table (table).
    tbody HtmlTableSectionElement [] true,
    /// The \`<td>\` HTML element defines a cell of a table that contains data. It participates in the table model.
    td HtmlTableCellElement [colspan, headers, rowspan] true,
    /// The \`<template>\` HTML element is a mechanism for holding HTML that is not to be rendered immediately when a page is loaded but may be instantiated subsequently during runtime using JavaScript.
    template HtmlTemplateElement [] true,
    /// The \`<textarea>\` HTML element represents a multi-line plain-text editing control, useful when you want to allow users to enter a sizeable amount of free-form text, for example a comment on a review or feedback form.
    textarea HtmlTextAreaElement [autocomplete, cols, dirname, disabled, form, maxlength, minlength, name, placeholder, readonly, required, rows, wrap] false,
    /// The \`<tfoot>\` HTML element defines a set of rows summarizing the columns of the table.
    tfoot HtmlTableSectionElement [] true,
    /// The \`<th>\` HTML element defines a cell as header of a group of table cells. The exact nature of this group is defined by the scope and headers attributes.
    th HtmlTableCellElement [abbr, colspan, headers, rowspan, scope] true,
    /// The \`<thead>\` HTML element defines a set of rows defining the head of the columns of the table.
    thead HtmlTableSectionElement [] true,
    /// The \`<time>\` HTML element represents a specific period in time. It may include the datetime attribute to translate dates into machine-readable format, allowing for better search engine results or custom features such as reminders.
    time HtmlTimeElement [datetime] true,
    ///	The \`<title>\` HTML element defines the document's title that is shown in a Browser's title bar or a page's tab. It only contains text; tags within the element are ignored.
    title HtmlTitleElement [] true,
    /// The \`<tr>\` HTML element defines a row of cells in a table. The row's cells can then be established using a mix of td (data cell) and th (header cell) elements.
    tr HtmlTableRowElement [] true,
    /// The \`<u>\` HTML element represents a span of inline text which should be rendered in a way that indicates that it has a non-textual annotation. This is rendered by default as a simple solid underline, but may be altered using CSS.
    u HtmlElement [] true,
    /// The \`<ul>\` HTML element represents an unordered list of items, typically rendered as a bulleted list.
    ul HtmlUListElement [] true,
    /// The \`<var>\` HTML element represents the name of a variable in a mathematical expression or a programming context. It's typically presented using an italicized version of the current typeface, although that behavior is browser-dependent.
    var HtmlElement [] true,
    /// The \`<video>\` HTML element embeds a media player which supports video playback into the document. You can use \`<video>\` for audio content as well, but the audio element may provide a more appropriate user experience.
    video HtmlVideoElement [autoplay, controls, controlslist, crossorigin, disablepictureinpicture, disableremoteplayback, height, r#loop, muted, playsinline, poster, preload, src, width] true,
	/// The \`<option>\` HTML element is used to define an item contained in a \`<select>\`, an\` <optgroup>\`, or a \`<datalist>\` element. As such, \`<option>\` can represent menu items in popups and other lists of items in an HTML document.
    option HtmlOptionElement [disabled, label, selected, value] true
	/// The \`<area>\` HTML element defines an area inside an image map that has predefined clickable areas. An image map allows geometric areas on an image to be associated with Hyperlink.
    area HtmlAreaElement [alt, coords, download, href, hreflang, ping, rel, shape, target] true,
    /// The \`<base>\` HTML element specifies the base URL to use for all relative URLs in a document. There can be only one \`<base>\` element in a document.
    base HtmlBaseElement [href, target] true,
    /// The \`<br>\` HTML element produces a line break in text (carriage-return). It is useful for writing a poem or an address, where the division of lines is significant.
    br HtmlBrElement [] true,
    /// The \`<col>\` HTML element defines a column within a table and is used for defining common semantics on all common cells. It is generally found within a colgroup element.
    col HtmlTableColElement [span] true,
    /// The \`<embed>\` HTML element embeds external content at the specified point in the document. This content is provided by an external application or other source of interactive content such as a browser plug-in.
    embed HtmlEmbedElement [height, src, r#type, width] true,
    /// The \`<hr>\` HTML element represents a thematic break between paragraph-level elements: for example, a change of scene in a story, or a shift of topic within a section.
    hr HtmlHrElement [] true,
    /// The \`<img>\` HTML element embeds an image into the document.
    img HtmlImageElement [alt, attributionsrc, crossorigin, decoding, elementtiming, fetchpriority, height, ismap, loading, referrerpolicy, sizes, src, srcset, usemap, width] true,
    /// The \`<input>\` HTML element is used to create interactive controls for web-based forms in order to accept data from the user; a wide variety of types of input data and control widgets are available, depending on the device and user agent. The \`<input>\` element is one of the most powerful and complex in all of HTML due to the sheer number of combinations of input types and attributes.
    input HtmlInputElement [accept, alt, autocomplete, capture, checked, dirname, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, height, list, max, maxlength, min, minlength, multiple, name, pattern, placeholder, popovertarget, popovertargetaction, readonly, required, size, src, step, r#type, value, width] true,
    ///	The \`<link>\` HTML element specifies relationships between the current document and an external resource. This element is most commonly used to link to CSS, but is also used to establish site icons (both "favicon" style icons and icons for the home screen and apps on mobile devices) among other things.
    link HtmlLinkElement [r#as, blocking, crossorigin, fetchpriority, href, hreflang, imagesizes, imagesrcset, integrity, media, rel, referrerpolicy, sizes, r#type] true,
    ///	The \`<meta>\` HTML element represents Metadata that cannot be represented by other HTML meta-related elements, like base, link, script, style or title.
    meta HtmlMetaElement [charset, content, http_equiv, name] true,
    /// The \`<source>\` HTML element specifies multiple media resources for the picture, the audio element, or the video element. It is an empty element, meaning that it has no content and does not have a closing tag. It is commonly used to offer the same media content in multiple file formats in order to provide compatibility with a broad range of browsers given their differing support for image file formats and media file formats.
    source HtmlSourceElement [src, r#type, srcset, sizes, media, height, width] true,
    /// The \`<track>\` HTML element is used as a child of the media elements, audio and video. It lets you specify timed text tracks (or time-based data), for example to automatically handle subtitles. The tracks are formatted in WebVTT format (.vtt files) — Web Video Text Tracks.
    track HtmlTrackElement [default, kind, label, src, srclang] true,
    /// The \`<wbr>\` HTML element represents a word break opportunity—a position within text where the browser may optionally break a line, though its line-breaking rules would not otherwise create a break at that location.
    wbr HtmlElement [] true,
`;

let attr_src = `
	/// The \`abbr\` attribute specifies an abbreviated form of the element's content.
	abbr "abbr",
    /// The \`accept-charset\` attribute specifies the character encodings that are to be used for the form submission.
    accept_charset "accept-charset",
    /// The \`accept\` attribute specifies a list of types the server accepts, typically a file type.
    accept "accept",
    /// The \`accesskey\` attribute specifies a shortcut key to activate or focus an element.
    accesskey "accesskey",
    /// The \`action\` attribute defines the URL to which the form data will be sent.
    action "action",
    /// The \`align\` attribute specifies the alignment of an element.
    align "align",
    /// The \`allow\` attribute defines a feature policy for the content in an iframe.
    allow "allow",
    /// The \`allowfullscreen\` attribute allows the iframe to be displayed in fullscreen mode.
    allowfullscreen "allowfullscreen",
    /// The \`allowpaymentrequest\` attribute allows a cross-origin iframe to invoke the Payment Request API.
    allowpaymentrequest "allowpaymentrequest",
    /// The \`alt\` attribute provides alternative text for an image, if the image cannot be displayed.
    alt "alt",
    /// The \`aria-activedescendant\` attribute identifies the currently active element when DOM focus is on a composite widget, textbox, group, or application.
    aria_activedescendant "aria-activedescendant",
    /// The \`aria-atomic\` attribute indicates whether assistive technologies will present all, or only parts of, the changed region based on the change notifications defined by the aria-relevant attribute.
    aria_atomic "aria-atomic",
    /// The \`aria-autocomplete\` attribute indicates whether user input completion suggestions are provided.
    aria_autocomplete "aria-autocomplete",
    /// The \`aria-busy\` attribute indicates whether an element, and its subtree, are currently being updated.
    aria_busy "aria-busy",
    /// The \`aria-checked\` attribute indicates the current "checked" state of checkboxes, radio buttons, and other widgets.
    aria_checked "aria-checked",
    /// The \`aria-colcount\` attribute defines the total number of columns in a table, grid, or treegrid.
    aria_colcount "aria-colcount",
    /// The \`aria-colindex\` attribute defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid.
    aria_colindex "aria-colindex",
    /// The \`aria-colspan\` attribute defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid.
    aria_colspan "aria-colspan",
    /// The \`aria-controls\` attribute identifies the element (or elements) whose contents or presence are controlled by the current element.
    aria_controls "aria-controls",
    /// The \`aria-current\` attribute indicates the element representing the current item within a container or set of related elements.
    aria_current "aria-current",
    /// The \`aria-describedby\` attribute identifies the element (or elements) that describes the object.
    aria_describedby "aria-describedby",
    /// The \`aria-description\` attribute provides a string value that describes or annotates the current element.
    aria_description "aria-description",
    /// The \`aria-details\` attribute identifies the element that provides a detailed, extended description for the object.
    aria_details "aria-details",
    /// The \`aria-disabled\` attribute indicates that the element is perceivable but disabled, so it is not editable or otherwise operable.
    aria_disabled "aria-disabled",
    /// The \`aria-dropeffect\` attribute indicates what functions can be performed when a dragged object is released on the drop target.
    aria_dropeffect "aria-dropeffect",
    /// The \`aria-errormessage\` attribute identifies the element that provides an error message for the object.
    aria_errormessage "aria-errormessage",
    /// The \`aria-expanded\` attribute indicates whether an element, or another grouping element it controls, is currently expanded or collapsed.
    aria_expanded "aria-expanded",
    /// The \`aria-flowto\` attribute identifies the next element (or elements) in an alternate reading order of content.
    aria_flowto "aria-flowto",
    /// The \`aria-grabbed\` attribute indicates an element's "grabbed" state in a drag-and-drop operation.
    aria_grabbed "aria-grabbed",
    /// The \`aria-haspopup\` attribute indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element.
    aria_haspopup "aria-haspopup",
    /// The \`aria-hidden\` attribute indicates whether the element is exposed to an accessibility API.
    aria_hidden "aria-hidden",
    /// The \`aria-invalid\` attribute indicates the entered value does not conform to the format expected by the application.
    aria_invalid "aria-invalid",
    /// The \`aria-keyshortcuts\` attribute indicates keyboard shortcuts that an author has implemented to activate or give focus to an element.
    aria_keyshortcuts "aria-keyshortcuts",
    /// The \`aria-label\` attribute defines a string value that labels the current element.
    aria_label "aria-label",
    /// The \`aria-labelledby\` attribute identifies the element (or elements) that labels the current element.
    aria_labelledby "aria-labelledby",
    /// The \`aria-live\` attribute indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region.
    aria_live "aria-live",
    /// The \`aria-modal\` attribute indicates whether an element is modal when displayed.
    aria_modal "aria-modal",
    /// The \`aria-multiline\` attribute indicates whether a text box accepts multiple lines of input or only a single line.
    aria_multiline "aria-multiline",
    /// The \`aria-multiselectable\` attribute indicates that the user may select more than one item from the current selectable descendants.
    aria_multiselectable "aria-multiselectable",
    /// The \`aria-orientation\` attribute indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    aria_orientation "aria-orientation",
    /// The \`aria-owns\` attribute identifies an element (or elements) in order to define a relationship between the element with \`aria-owns\` and the target element.
    aria_owns "aria-owns",
    /// The \`aria-placeholder\` attribute defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value.
    aria_placeholder "aria-placeholder",
    /// The \`aria-posinset\` attribute defines an element's position within a set or treegrid.
    aria_posinset "aria-posinset",
    /// The \`aria-pressed\` attribute indicates the current "pressed" state of toggle buttons.
    aria_pressed "aria-pressed",
    /// The \`aria-readonly\` attribute indicates that the element is not editable, but is otherwise operable.
    aria_readonly "aria-readonly",
    /// The \`aria-relevant\` attribute indicates what user agent changes to the accessibility tree should be monitored.
    aria_relevant "aria-relevant",
    /// The \`aria-required\` attribute indicates that user input is required on the element before a form may be submitted.
    aria_required "aria-required",
    /// The \`aria-roledescription\` attribute defines a human-readable, author-localized description for the role of an element.
    aria_roledescription "aria-roledescription",
    /// The \`aria-rowcount\` attribute defines the total number of rows in a table, grid, or treegrid.
    aria_rowcount "aria-rowcount",
    /// The \`aria-rowindex\` attribute defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid.
    aria_rowindex "aria-rowindex",
    /// The \`aria-rowspan\` attribute defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
    aria_rowspan "aria-rowspan",
    /// The \`aria-selected\` attribute indicates the current "selected" state of various widgets.
    aria_selected "aria-selected",
    /// The \`aria-setsize\` attribute defines the number of items in the current set of listitems or treeitems.
    aria_setsize "aria-setsize",
    /// The \`aria-sort\` attribute indicates if items in a table or grid are sorted in ascending or descending order.
    aria_sort "aria-sort",
    /// The \`aria-valuemax\` attribute defines the maximum allowed value for a range widget.
    aria_valuemax "aria-valuemax",
    /// The \`aria-valuemin\` attribute defines the minimum allowed value for a range widget.
    aria_valuemin "aria-valuemin",
    /// The \`aria-valuenow\` attribute defines the current value for a range widget.
    aria_valuenow "aria-valuenow",
    /// The \`aria-valuetext\` attribute defines the human-readable text alternative of aria-valuenow for a range widget.
    aria_valuetext "aria-valuetext",
    /// The \`as\` attribute specifies the type of destination for the content of the link.
    r#as "as",
    /// The \`async\` attribute indicates that the script should be executed asynchronously.
    r#async "async",
    /// The \`attributionsrc\` attribute indicates that you want the browser to send an \`Attribution-Reporting-Eligible\` header along with a request.
    attributionsrc "attributionsrc",
    /// The \`autocapitalize\` attribute controls whether and how text input is automatically capitalized as it is entered/edited by the user.
    autocapitalize "autocapitalize",
    /// The \`autocomplete\` attribute indicates whether an input field can have its value automatically completed by the browser.
    autocomplete "autocomplete",
    /// The \`autofocus\` attribute indicates that an element should be focused on page load.
    autofocus "autofocus",
    /// The \`autoplay\` attribute indicates that the media should start playing as soon as it is loaded.
    autoplay "autoplay",
    /// The \`background\` attribute sets the URL of the background image for the document.
    background "background",
    /// The \`bgcolor\` attribute sets the background color of an element.
    bgcolor "bgcolor",
    /// The \`blocking\` attribute indicates that the script will block the page loading until it is executed.
    blocking "blocking",
    /// The \`border\` attribute sets the width of an element's border.
    border "border",
    /// The \`buffered\` attribute contains the time ranges that the media has been buffered.
    buffered "buffered",
    /// The \`capture\` attribute indicates that the user must capture media using a camera or microphone instead of selecting a file from the file picker.
    capture "capture",
    /// The \`challenge\` attribute specifies the challenge string that is paired with the keygen element.
    challenge "challenge",
    /// The \`closedby\` attribute specifies the types of user actions that can be used to close the associated \`<dialog>\` element.
    closedby "closedby",
    /// The \`charset\` attribute specifies the character encoding of the HTML document.
    charset "charset",
    /// The \`checked\` attribute indicates whether an input element is checked or not.
    checked "checked",
    /// The \`cite\` attribute contains a URL that points to the source of the quotation or change.
    cite "cite",
    /// The \`class\` attribute is a space-separated list of the classes of the element
	class class
    /// The \`code\` attribute specifies the URL of the applet's class file to be loaded and executed.
    code "code",
    /// The \`color\` attribute specifies the color of an element's text.
    color "color",
    /// The \`cols\` attribute specifies the visible width of a text area.
    cols "cols",
    /// The \`colspan\` attribute defines the number of columns a cell should span.
    colspan "colspan",
    /// The \`command\` attribute defines the command to be invoked when user clicks the \`<button>\` element which has \`commandfor\` attribute specified.
    command "command",
    /// The \`commandfor\` attribute defines the id of the element which button is controlling. It is generic version of \`popovertarget\`.
    commandfor "commandfor",
    /// The \`content\` attribute gives the value associated with the http-equiv or name attribute.
    content "content",
    /// The \`contenteditable\` attribute indicates whether the element's content is editable.
    contenteditable "contenteditable",
    /// The \`contextmenu\` attribute specifies the ID of a \`<menu>\` element to open as a context menu.
    contextmenu "contextmenu",
    /// The \`controls\` attribute indicates whether the browser should display playback controls for the media.
    controls "controls",
    /// The \`controlslist\` attribute allows the control of which controls to show on the media element whenever the browser shows its native controls.
    controlslist "controlslist",
    /// The \`coords\` attribute specifies the coordinates of an area in an image map.
    coords "coords",
    /// The \`crossorigin\` attribute indicates whether the resource should be fetched with a CORS request.
    crossorigin "crossorigin",
    /// The \`csp\` attribute allows the embedding document to define the Content Security Policy that an embedded document must agree to enforce upon itself.
    csp "csp",
    /// The \`data\` attribute specifies the URL of the resource that is being embedded.
    data "data",
    /// The \`datetime\` attribute specifies the date and time.
    datetime "datetime",
    /// The \`decoding\` attribute indicates the preferred method for decoding images.
    decoding "decoding",
    /// The \`default\` attribute indicates that the track should be enabled unless the user's preferences indicate that another track is more appropriate.
    default "default",
    /// The \`defer\` attribute indicates that the script should be executed after the document has been parsed.
    defer "defer",
    /// The \`dir\` attribute specifies the text direction for the content in an element.
    dir "dir",
    /// The \`dirname\` attribute identifies the text directionality of an input element.
    dirname "dirname",
    /// The \`disabled\` attribute indicates whether the element is disabled.
    disabled "disabled",
    /// The \`disablepictureinpicture\` attribute indicates that the element is not allowed to be displayed in Picture-in-Picture mode.
    disablepictureinpicture "disablepictureinpicture",
    /// The \`disableremoteplayback\` attribute indicates that the element is not allowed to be displayed using remote playback.
    disableremoteplayback "disableremoteplayback",
    /// The \`download\` attribute indicates that the linked resource is intended to be downloaded rather than displayed in the browser.
    download "download",
    /// The \`draggable\` attribute indicates whether the element is draggable.
    draggable "draggable",
    /// The \`elementtiming\` attributes marks the element for observation by the \`PerformanceElementTiming\` API.
    elementtiming "elementtiming",
    /// The \`enctype\` attribute specifies the MIME type of the form submission.
    enctype "enctype",
    /// The \`enterkeyhint\` attribute allows authors to specify what kind of action label or icon will be presented to users in a virtual keyboard's enter key.
    enterkeyhint "enterkeyhint",
    /// The \`exportparts\` attribute enables the sharing of parts of an element's shadow DOM with a containing document.
    exportparts "exportparts",
    /// The \`fetchpriority\` attribute allows developers to specify the priority of a resource fetch request.
    fetchpriority "fetchpriority",
    /// The \`for\` attribute specifies which form element a label is bound to.
    r#for "for",
    /// The \`form\` attribute associates the element with a form element.
    form "form",
    /// The \`formaction\` attribute specifies the URL that processes the form submission.
    formaction "formaction",
    /// The \`formenctype\` attribute specifies how the form data should be encoded when submitted.
    formenctype "formenctype",
    /// The \`formmethod\` attribute specifies the HTTP method to use when submitting the form.
    formmethod "formmethod",
    /// The \`formnovalidate\` attribute indicates that the form should not be validated when submitted.
    formnovalidate "formnovalidate",
    /// The \`formtarget\` attribute specifies where to display the response after submitting the form.
    formtarget "formtarget",
    /// The \`headers\` attribute specifies the headers associated with the element.
    headers "headers",
    /// The \`height\` attribute specifies the height of an element.
    height "height",
    /// The \`hidden\` attribute indicates that the element is not yet, or is no longer, relevant.
    hidden "hidden",
    /// The \`high\` attribute specifies the range that is considered to be a high value.
    high "high",
    /// The \`href\` attribute specifies the URL of a linked resource.
    href "href",
    /// The \`hreflang\` attribute specifies the language of the linked resource.
    hreflang "hreflang",
    /// The \`http-equiv\` attribute provides an HTTP header for the information/value of the content attribute.
    http_equiv "http-equiv",
    /// The \`icon\` attribute specifies the URL of an image to be used as a graphical icon for the element.
    icon "icon",
    /// The \`id\` attribute specifies a unique id for an element.
    id "id",
    /// The \`imagesizes\` attribute specifies image sizes for different page layouts.
    imagesizes "imagesizes",
    /// The \`imagesrcset\` attribute specifies the URLs of multiple images to be used in different situations.
    imagesrcset "imagesrcset",
    /// The \`importance\` attribute specifies the relative importance of the element.
    importance "importance",
    /// The \`inert\` attribute indicates that the element is non-interactive and won't be accessible to user interactions or assistive technologies.
    inert "inert",
    /// The \`inputmode\` attribute specifies the type of data that the user will enter.
    inputmode "inputmode",
    /// The \`integrity\` attribute contains a hash value that the browser can use to verify that the resource hasn't been altered.
    integrity "integrity",
    /// The \`intrinsicsize\` attribute specifies the intrinsic size of an image or video.
    intrinsicsize "intrinsicsize",
    /// The \`is\` attribute allows you to specify the name of a custom element.
    is "is",
    /// The \`ismap\` attribute indicates that the image is part of a server-side image map.
    ismap "ismap",
    /// The \`itemid\` attribute assigns a unique identifier to an item.
    itemid "itemid",
    /// The \`itemprop\` attribute adds a property to an item.
    itemprop "itemprop",
    /// The \`itemref\` attribute provides a list of element IDs that have additional properties for the item.
    itemref "itemref",
    /// The \`itemscope\` attribute creates a new item and adds it to the page's items.
    itemscope "itemscope",
    /// The \`itemtype\` attribute specifies the type of an item.
    itemtype "itemtype",
    /// The \`keytype\` attribute specifies the type of key used by the \`<keygen>\` element.
    keytype "keytype",
    /// The \`kind\` attribute specifies the kind of text track.
    kind "kind",
    /// The \`label\` attribute provides a user-readable title for an element.
    label "label",
    /// The \`lang\` attribute specifies the language of the element's content.
    lang "lang",
    /// The \`language\` attribute specifies the scripting language used for the script.
    language "language",
    /// The \`list\` attribute identifies a \`<datalist>\` element that contains pre-defined options for an \`<input>\` element.
    list "list",
    /// The \`loading\` attribute indicates how the browser should load the image.
    loading "loading",
    /// The \`loop\` attribute indicates whether the media should start over again when it reaches the end.
    r#loop "loop",
    /// The \`low\` attribute specifies the range that is considered to be a low value.
    low "low",
    /// The \`manifest\` attribute specifies the URL of a document's cache manifest.
    manifest "manifest",
    /// The \`max\` attribute specifies the maximum value for an input element.
    max "max",
    /// The \`maxlength\` attribute specifies the maximum number of characters that an input element can accept.
    maxlength "maxlength",
    /// The \`media\` attribute specifies what media/device the linked resource is optimized for.
    media "media",
    /// The \`method\` attribute specifies the HTTP method to use when submitting the form.
    method "method",
    /// The \`min\` attribute specifies the minimum value for an input element.
    min "min",
    /// The \`minlength\` attribute specifies the minimum number of characters that an input element can accept.
    minlength "minlength",
    /// The \`multiple\` attribute indicates whether the user can enter more than one value.
    multiple "multiple",
    /// The \`muted\` attribute indicates whether the audio will be initially silenced on page load.
    muted "muted",
    /// The \`name\` attribute specifies the name of the element.
    name "name",
    /// The \`nomodule\` attribute indicates that the script should not be executed in browsers that support ES modules.
    nomodule "nomodule",
    /// The \`nonce\` attribute provides a cryptographic nonce to ensure that a script or style is approved for execution.
    nonce "nonce",
    /// The \`novalidate\` attribute indicates that the form should not be validated when submitted.
    novalidate "novalidate",
    /// The \`open\` attribute indicates whether the details element is open or closed.
    open "open",
    /// The \`optimum\` attribute specifies the range that is considered to be an optimum value.
    optimum "optimum",
    /// The \`part\` attribute identifies the element as a shadow DOM part.
    part "part",
    /// The \`pattern\` attribute specifies a regular expression that the input element's value is checked against.
    pattern "pattern",
    /// The \`ping\` attribute contains a space-separated list of URLs to be notified if the user follows the hyperlink.
    ping "ping",
    /// The \`placeholder\` attribute provides a short hint that describes the expected value of the input element.
    placeholder "placeholder",
    /// The \`playsinline\` attribute indicates that the video should play inline in the element's playback area.
    playsinline "playsinline",
    /// The \`popover\` attribute indicates that an element is a popover and specifies the event that causes the popover to be shown.
    popover "popover",
    /// The \`popovertarget\` attribute specifies the ID of an element to toggle a popover.
    popovertarget "popovertarget",
    /// The \`popovertargetaction\` attribute specifies the action that shows the popover.
    popovertargetaction "popovertargetaction",
    /// The \`poster\` attribute specifies an image to be shown while the video is downloading or until the user hits the play button.
    poster "poster",
    /// The \`preload\` attribute specifies if and how the author thinks that the media file should be loaded when the page loads.
    preload "preload",
    /// The \`radiogroup\` attribute specifies the name of the group to which the element belongs.
    radiogroup "radiogroup",
    /// The \`readonly\` attribute indicates that the user cannot modify the value of the input element.
    readonly "readonly",
    /// The \`referrerpolicy\` attribute specifies which referrer information to include with requests.
    referrerpolicy "referrerpolicy",
    /// The \`rel\` attribute specifies the relationship between the current document and the linked document.
    rel "rel",
    /// The \`required\` attribute indicates that the user must fill in the input element before submitting the form.
    required "required",
    /// The \`reversed\` attribute indicates that the list should be displayed in a descending order.
    reversed "reversed",
    /// The \`role\` attribute defines the role of an element in the context of a web application.
    role "role",
    /// The \`rows\` attribute specifies the number of visible text lines for a text area.
    rows "rows",
    /// The \`rowspan\` attribute defines the number of rows a cell should span.
    rowspan "rowspan",
    /// The \`sandbox\` attribute applies extra restrictions to the content in the \`<iframe>\`.
    sandbox "sandbox",
    /// The \`scope\` attribute specifies whether a header cell is a header for a column, row, or group of columns or rows.
    scope "scope",
    /// The \`scoped\` attribute indicates that the styles in a \`<style>\` element are scoped to the parent element.
    scoped "scoped",
    /// The \`selected\` attribute indicates that the option is selected.
    selected "selected",
    /// The \`shape\` attribute specifies the shape of the area.
    shape "shape",
    /// The \`size\` attribute specifies the width of the input element.
    size "size",
    /// The \`sizes\` attribute specifies the sizes of icons for visual media.
    sizes "sizes",
    /// The \`slot\` attribute assigns a slot to an element.
    slot "slot",
    /// The \`span\` attribute defines the number of columns in a \`<colgroup>\` or the number of rows in a \`<rowgroup>\`.
    span "span",
    /// The \`spellcheck\` attribute indicates whether spell checking is allowed for the element.
    spellcheck "spellcheck",
    /// The \`src\` attribute specifies the URL of the media resource.
    src "src",
    /// The \`srcdoc\` attribute specifies the HTML content of the page to show in the \`<iframe>\`.
    srcdoc "srcdoc",
    /// The \`srclang\` attribute specifies the language of the text track.
    srclang "srclang",
    /// The \`srcset\` attribute specifies the URLs of multiple images to be used in different situations.
    srcset "srcset",
    /// The \`start\` attribute specifies the start value of the list.
    start "start",
    /// The \`step\` attribute specifies the legal number intervals for an input element.
    step "step",
    /// The \`style\` attribute specifies inline CSS styling declarations for an element.
    style style
    /// The \`summary\` attribute provides a summary of the content of the table.
    summary "summary",
    /// The \`tabindex\` attribute specifies the tab order of an element.
    tabindex "tabindex",
    /// The \`target\` attribute specifies where to open the linked document.
    target "target",
    /// The \`title\` attribute provides additional information about an element.
    title "title",
    /// The \`translate\` attribute specifies whether the content of an element should be translated or not.
    translate "translate",
    /// The \`type\` attribute specifies the type of the element.
    r#type "type",
    /// The \`usemap\` attribute specifies the image map to be used by an \`<img>\` element.
    usemap "usemap",
    /// The \`value\` attribute specifies the value of the element.
    value "value",
    /// The \`virtualkeyboardpolicy\` attribute controls the policy for virtual keyboards.
    virtualkeyboardpolicy "virtualkeyboardpolicy",
    /// The \`width\` attribute specifies the width of an element.
    width "width",
    /// The \`wrap\` attribute specifies how the text in a text area is to be wrapped when submitted in a form.
    wrap "wrap",
`;

function chunk<T>(arr: T[], size: number): T[][] {
	let chunks = [];
	for (let i = 0; i < arr.length; i += size) {
		chunks.push(arr.slice(i, i + size));
	}
	return chunks;
}

let tag_results: { tag: string, attrs: string[], doc: string }[] = [];
let tag_parts = chunk(html_src.trim().split("\n").map(v => v.trim()), 2);
for (let part of tag_parts) {
	let [doc, spec] = part;
	
	let [tag, _, ...rest] = spec.split(" ");
	let attrs = rest.join(" ").slice(1).split(']')[0].split(',').map(v => v.trim()).filter(Boolean);
	doc = doc.slice(4).replace(/`[^`]+`/, v => `[${v}](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/${v.slice(2, -2)})`);
	tag_results.push({ tag, attrs, doc })
}

tag_results.push({ tag: "svg", attrs: [], doc: `The [\`<svg>\`](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg) SVG element is a container defining a new coordinate system and viewport. It is used as the outermost element of SVG documents, but it can also be used to embed an SVG fragment inside an SVG or HTML document.` });

tag_results.push({ tag: "math", attrs: [], doc: `The [\`<math>\`](https://developer.mozilla.org/en-US/docs/Web/MathML/Reference/Element/math) MathML element is the top-level MathML element, used to write a single mathematical formula. It can be placed in HTML content where flow content is permitted.` });

tag_results.sort((a, b) => a.tag.localeCompare(b.tag));

Deno.writeTextFileSync("html_tags.txt", tag_results.map(
	v => `\t/// ${v.doc}\n\tpub struct ${v.tag};`
).join("\n"));

Deno.writeTextFileSync("tags.txt", tag_results.map(v => v.tag).join(", "));

let global_attr = new Set(["accesskey","autocapitalize","autofocus","class","contenteditable","dir","draggable","enterkeyhint","exportparts","hidden","id","inert","inputmode","is","itemid","itemprop","itemref","itemscope","itemtype","lang","nonce","part","popover","slot","style","spellcheck","tabindex","title","translate","virtualkeyboardpolicy"]);

let attr_results: { name: string, doc: string }[] = [];
let attr_parts = chunk(attr_src.trim().split("\n").map(v => v.trim()), 2);
for (let part of attr_parts) {
	let [doc, spec] = part;
	
	let attr = spec.split(" ")[0];
	doc = doc.slice(4);
	if (global_attr.has(attr)) doc =  doc.replace(/`[^`]+`/, v => `[${v}](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/${v.slice(1, -1)})`);
	if (attr == "role" || attr.startsWith("aria")) doc =  doc.replace(/`[^`]+`/, v => `[${v}](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/${v.slice(1, -1)})`);
	attr_results.push({ name: attr, doc });
}
attr_results.sort((a, b) => a.name.localeCompare(b.name));

Deno.writeTextFileSync("html_attrs.txt", attr_results.map(
	v => `\t/// ${v.doc}\n\tpub struct ${v.name};`
).join("\n"));

console.log("global_attr: ", attr_results.filter(v => global_attr.has(v.name) || v.name == "role" || v.name.startsWith("aria")).map(v => v.name).join(","));