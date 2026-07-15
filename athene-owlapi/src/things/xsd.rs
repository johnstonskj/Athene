use rdftk_iri::vocab::VOCABULARY_XML_SCHEMA;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Datatypes
// ------------------------------------------------------------------------------------------------

// Section 4.1 Real Numbers, Decimal Numbers, and Integers

make_iri_function!(decimal => VOCABULARY_XML_SCHEMA:decimal);
make_iri_function!(integer => VOCABULARY_XML_SCHEMA:integer);
make_iri_function!(non_negative_integer => VOCABULARY_XML_SCHEMA:nonNegativeInteger);
make_iri_function!(non_positive_integer => VOCABULARY_XML_SCHEMA:nonPositiveInteger);
make_iri_function!(positive_integer => VOCABULARY_XML_SCHEMA:positiveInteger);
make_iri_function!(negative_integer => VOCABULARY_XML_SCHEMA:negativeInteger);
make_iri_function!(long => VOCABULARY_XML_SCHEMA:long);
make_iri_function!(int => VOCABULARY_XML_SCHEMA:int);
make_iri_function!(short => VOCABULARY_XML_SCHEMA:short);
make_iri_function!(byte => VOCABULARY_XML_SCHEMA:byte);
make_iri_function!(unsigned_long => VOCABULARY_XML_SCHEMA:unsignedLong);
make_iri_function!(unsigned_int => VOCABULARY_XML_SCHEMA:unsignedInt);
make_iri_function!(unsigned_short => VOCABULARY_XML_SCHEMA:unsignedShort);
make_iri_function!(unsigned_byte => VOCABULARY_XML_SCHEMA:unsignedByte);

// Section 4.2 Floating-Point Numbers

make_iri_function!(double => VOCABULARY_XML_SCHEMA:double);
make_iri_function!(float => VOCABULARY_XML_SCHEMA:float);

// Section 4.3 Strings

make_iri_function!(string => VOCABULARY_XML_SCHEMA:string);
make_iri_function!(normalized_string => VOCABULARY_XML_SCHEMA:normalizedString);
make_iri_function!(token => VOCABULARY_XML_SCHEMA:token);
make_iri_function!(language => VOCABULARY_XML_SCHEMA:language);
make_iri_function!(name => VOCABULARY_XML_SCHEMA:Name);
make_iri_function!(nc_name => VOCABULARY_XML_SCHEMA:NCName);
make_iri_function!(nm_token => VOCABULARY_XML_SCHEMA:NMTOKEN);

// Section 4.4 Boolean Values

make_iri_function!(boolean => VOCABULARY_XML_SCHEMA:boolean);

// Section 4.5 Binary Data

make_iri_function!(base64_binary => VOCABULARY_XML_SCHEMA:base64Binary);
make_iri_function!(hex_binary => VOCABULARY_XML_SCHEMA:hexBinary);

// Section 4.6 IRIs

make_iri_function!(any_uri => VOCABULARY_XML_SCHEMA:anyURI);

// Section 4.7 Time Instants

make_iri_function!(date_time => VOCABULARY_XML_SCHEMA:dateTime);
make_iri_function!(date_time_stamp => VOCABULARY_XML_SCHEMA:dateTimeStamp);
