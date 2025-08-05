// Test file for JSX property sorting

const BasicComponent = () => {
    return (
        <div 
            className="container"
            onClick={handleClick}
            id="main"
            style={styles}
            key="unique"
            ref={divRef}
            data-testid="test"
        />
    );
};

const ButtonWithEvents = () => (
    <button
        type="submit"
        onMouseEnter={handleEnter}
        className="btn btn-primary"
        onClick={handleClick}
        disabled={isDisabled}
        onMouseLeave={handleLeave}
        aria-label="Submit form"
        onChange={handleChange}
        key="submit-btn"
        onFocus={handleFocus}
        id="submit"
        ref={buttonRef}
    />
);

const ComponentWithSpread = (props) => (
    <div
        {...defaultProps}
        className="card"
        id={props.id}
        {...props}
        style={customStyle}
        ref={cardRef}
        key={props.key}
        onClick={props.onClick}
        data-value={props.value}
        {...restProps}
    />
);

const FormElements = () => (
    <form>
        <input
            value={inputValue}
            onChange={handleInputChange}
            placeholder="Enter text"
            type="text"
            name="username"
            id="username-input"
            ref={inputRef}
            key="username"
            onBlur={handleBlur}
            onFocus={handleFocus}
            required
            autoComplete="username"
        />
        
        <select
            value={selectedValue}
            onChange={handleSelectChange}
            name="options"
            id="options-select"
            key="options"
            ref={selectRef}
            multiple={false}
            disabled={!hasOptions}
        >
            <option value="1">Option 1</option>
            <option value="2">Option 2</option>
        </select>
    </form>
);

const NestedComponents = () => {
    return (
        <div className="wrapper" key="wrapper">
            <header ref={headerRef} className="header" id="main-header">
                <nav>
                    <a 
                        href="/home"
                        onClick={handleNavClick}
                        className="nav-link"
                        key="home-link"
                        ref={homeLinkRef}
                        target="_blank"
                        rel="noopener"
                    >
                        Home
                    </a>
                </nav>
            </header>
        </div>
    );
};