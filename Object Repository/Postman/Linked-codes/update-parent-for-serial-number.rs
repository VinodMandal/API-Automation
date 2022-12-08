<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update-parent-for-serial-number</name>
   <tag></tag>
   <elementGuidId>d7f2353b-4ace-4879-b805-c82bccf8d155</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;child_serial_number\&quot;:[10,12],\n\t\&quot;parent_id\&quot;:11\n}&quot;,
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
   <restUrl>${Host}linked-codes/update-parent/</restUrl>
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
      <id>3114e3a9-655f-4991-a373-bbca151329a9</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Id</defaultValue>
      <description></description>
      <id>706beb1f-8705-47b5-a504-176647c2ccb1</id>
      <masked>false</masked>
      <name>Auth Id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Token</defaultValue>
      <description></description>
      <id>30e1b526-60af-4874-a8fe-e2cf7aeb47be</id>
      <masked>false</masked>
      <name>Auth Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PK</defaultValue>
      <description></description>
      <id>df6f1209-bd75-42d0-adc7-235142f44323</id>
      <masked>false</masked>
      <name>PK</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
