<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Postman Request(4)</name>
   <tag></tag>
   <elementGuidId>dee647ea-367a-4ab7-aeb1-dd50dda70264</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;ucode_data\&quot;:\&quot;e1~24_64,2ob8r099th03l1dt,2\&quot;,\n    \&quot;ucode_parameters\&quot;: [\n            {\n                \&quot;pattern\&quot;: \&quot;SecretCode\&quot;,\n                \&quot;value\&quot;: \&quot;3263\&quot;\n            }\n        ]\n    ,\n    \&quot;captured_frame_url\&quot;:\&quot;aws_path\&quot;,\n    \&quot;location\&quot;:{\&quot;latitude\&quot;:13.6551139,\&quot;longitude\&quot;:74.6648285},\n    \&quot;captured_frame_path\&quot;:\&quot;Android\\/com.acviss.app\\/7975652354\\/7975652354_17082020181440431.jpg\&quot;\n    }\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-id</name>
      <type>Main</type>
      <value>${Auth Id}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-token</name>
      <type>Main</type>
      <value>${Auth Token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>device-type</name>
      <type>Main</type>
      <value>ANDROID</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>mobile</name>
      <type>Main</type>
      <value>${mobile_number}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value></value>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Host}authenticity-verification/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Host</defaultValue>
      <description></description>
      <id>d3ac5ed9-be6a-49a5-ba3f-0dc9ae28674c</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Id</defaultValue>
      <description></description>
      <id>06ed76b1-f63e-41f6-94f3-7a31b107ef13</id>
      <masked>false</masked>
      <name>Auth Id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Token</defaultValue>
      <description></description>
      <id>2eda63ee-c89a-4782-98e6-4177d917935b</id>
      <masked>false</masked>
      <name>Auth Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobile_number</defaultValue>
      <description></description>
      <id>266d07de-5122-4738-92f5-8aff69d29744</id>
      <masked>false</masked>
      <name>mobile_number</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
