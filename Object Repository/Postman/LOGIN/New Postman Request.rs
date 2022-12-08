<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Postman Request</name>
   <tag></tag>
   <elementGuidId>80e59460-1150-4d4a-a02f-98a9ab4f02cd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;otp_type\&quot;: \&quot;sms\&quot;,\n    \&quot;mobile\&quot;: ${mobile_number},\n    \&quot;country_code\&quot;: \&quot;+91\&quot;,\n    \&quot;otp_token\&quot;: \&quot;${otp_token}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
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
      <name>mobile</name>
      <type>Main</type>
      <value>7975652354</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>customer_id</name>
      <type>Main</type>
      <value>64</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Host}get-otp/</restUrl>
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
      <id>2b38a00c-210e-4a09-a474-250b4beef390</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Id</defaultValue>
      <description></description>
      <id>2911acd7-866b-41e7-9dbe-767c246d0520</id>
      <masked>false</masked>
      <name>Auth Id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Token</defaultValue>
      <description></description>
      <id>4f65ab8d-90b7-4722-8b0c-4f68ebc4650f</id>
      <masked>false</masked>
      <name>Auth Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobile_number</defaultValue>
      <description></description>
      <id>28b958f4-3e69-4897-a8aa-0ca9be3710c1</id>
      <masked>false</masked>
      <name>mobile_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.otp_token</defaultValue>
      <description></description>
      <id>1c61ec27-2986-4f31-b147-19d34e90dbbd</id>
      <masked>false</masked>
      <name>otp_token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
