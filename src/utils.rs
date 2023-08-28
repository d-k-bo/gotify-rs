use url::Url;

pub(crate) trait UrlAppend {
    fn append(&self, segments: impl IntoIterator<Item = impl AsRef<str>>) -> Url;
}

impl UrlAppend for Url {
    fn append(&self, segments: impl IntoIterator<Item = impl AsRef<str>>) -> Url {
        let mut url = self.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .extend(segments);
        url
    }
}

#[cfg(any(feature = "app", feature = "client-core"))]
macro_rules! request_builder {
    (
        name = $name:ident,
        client_type = $client_type:ty,
        method = $method:expr,
        $( uri = $uri:expr, )?
        $( uri_with = $uri_with:expr, )?
        return_type = $return_type:tt,
        required_fields = {
            $( $( #[ $required_field_attrs:meta ] )* $required_field_name:ident : $required_field_setter_type:ty $( => . $required_field_setter_method:ident() )? => $required_field_type:ty ),* $(,)?
        },
        optional_fields = {
            $( $( #[ $optional_field_attrs:meta ] )* $optional_field_name:ident : $optional_field_setter_type:ty $( => . $optional_field_setter_method:ident() )? => $optional_field_type:ty ),* $(,)?
        } $(,)?
    ) => {
        #[allow(missing_docs)]
        #[derive(Debug, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name<'client> {
            #[serde(skip)]
            client: &'client $client_type,
            $(
                $(
                    #[$required_field_attrs]
                )*
                $required_field_name: $required_field_type,
            )*
            $(
                $(
                    #[$optional_field_attrs]
                )*
                $optional_field_name: Option<$optional_field_type>,
            )*
        }
        #[allow(missing_docs)]
        impl<'client> $name<'client> {
            #[allow(clippy::redundant_field_names)]
            pub fn new(client: &'client $client_type, $( $required_field_name: $required_field_setter_type ),*) -> Self {
                Self {
                    client,
                    $(
                        $required_field_name: $required_field_name $( .$required_field_setter_method() )?,
                    )*
                    $(
                        $optional_field_name: None,
                    )*
                }
            }
        }
        paste::paste! {
            #[allow(missing_docs)]
            impl<'client> $name<'client> {
                $(
                    pub fn [<with_ $optional_field_name>](mut self, $optional_field_name: $optional_field_setter_type) -> Self {
                        self.$optional_field_name = Some($optional_field_name $( .$optional_field_setter_method() )? );
                        self
                    }
                )*
            }
        }
        #[allow(missing_docs)]
        impl<'client> $name<'client> {
            #[allow(clippy::redundant_closure_call)]
            pub async fn send(self) -> crate::Result<$return_type> {
                let r = self.client.request($method, $( $uri )? $( $uri_with(&self) )?);
                let r = if $method == reqwest::Method::GET {
                    r.with_query(self)
                } else {
                    r.with_json_body(self)
                };
                crate::utils::_send_and_match_return_type!(r, $return_type)
            }
        }
        impl<'client> std::future::IntoFuture for $name<'client> {
            type Output = crate::Result<$return_type>;
            type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'client>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }
    };
}

#[cfg(any(feature = "app", feature = "client-core"))]
macro_rules! _send_and_match_return_type {
    ( $r:ident, () ) => {
        $r.send().await
    };
    ( $r:ident, String ) => {
        $r.send_and_read_string().await
    };
    ( $r:ident, $type:ty ) => {
        $r.send_and_read_json().await
    };
}

#[cfg(any(feature = "app", feature = "client-core"))]
pub(crate) use _send_and_match_return_type;
#[cfg(any(feature = "app", feature = "client-core"))]
pub(crate) use request_builder;
