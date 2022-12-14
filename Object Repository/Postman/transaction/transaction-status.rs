<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>transaction-status</name>
   <tag></tag>
   <elementGuidId>c40679b4-bf1c-447f-a9d0-5662ee83fd63</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;transaction_id\&quot;:1225555\n}&quot;,
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
   <restUrl>${Host}transaction/transaction-status/</restUrl>
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
      <id>28ef59e0-7ca3-4415-965f-cadd7aaa20b2</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PK</defaultValue>
      <description></description>
      <id>c65c8f5a-99f3-4b4c-a130-c73b389c52f8</id>
      <masked>false</masked>
      <name>PK</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
