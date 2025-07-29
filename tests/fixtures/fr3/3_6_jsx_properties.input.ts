// FR3.6: JSX properties should be sorted with special rules

const BasicComponent = () => (
    <Button 
        disabled
        onClick={handleClick}
        className="btn-primary"
        aria-label="Submit"
    />
);

// key and ref should come first
const ListItem = ({ item }) => (
    <Item
        name={item.name}
        key={item.id}
        className="list-item"
        ref={itemRef}
        onClick={() => selectItem(item)}
    />
);

// Event handlers should be grouped
const InteractiveForm = () => (
    <Input
        value={inputValue}
        onChange={handleChange}
        placeholder="Enter text"
        onClick={handleClick}
        onBlur={handleBlur}
        onFocus={handleFocus}
        className="form-input"
        disabled={isDisabled}
    />
);

// Spread props should be at the end
const ExtendedComponent = () => (
    <Component
        name="test"
        {...defaultProps}
        className="extended"
        id="comp-1"
        {...overrideProps}
    />
);