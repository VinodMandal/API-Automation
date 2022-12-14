<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post-points-approval-request</name>
   <tag></tag>
   <elementGuidId>7bde4375-cdb3-48fc-857d-de6909e71751</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;json_data\&quot;: {\n        \&quot;external_user_id\&quot;: \&quot;1\&quot;,\n        \&quot;loyalty_user_code\&quot;: \&quot;wk40p2ln\&quot;,\n        \&quot;loyalty_user_points\&quot;: [\n            {\n                \&quot;loyalty_user_points_id\&quot;: 1\n            }\n        ]\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-id</name>
      <type>Main</type>
      <value>4A29948EB168F23543E2</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-token</name>
      <type>Main</type>
      <value>9B5DB42E5E91E64B749CA5CD4FF0B1075FD77471</value>
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
      <value>${PK}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>customer_id</name>
      <type>Main</type>
      <value>155</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Host}loyalty-points/post-points-approval-request/</restUrl>
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
      <id>eae26a6e-5cc4-4042-a124-f44c7ce19a0f</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PK</defaultValue>
      <description></description>
      <id>1627dd3b-423e-454f-abd5-6a29ea29f2e7</id>
      <masked>false</masked>
      <name>PK</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
