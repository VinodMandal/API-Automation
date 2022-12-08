<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name> Get-otp</name>
   <tag></tag>
   <elementGuidId>9e9357f4-9dc6-47cb-8baa-a0f6a8533747</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;otp_type\&quot;: \&quot;sms\&quot;,\n    \&quot;mobile\&quot;: \&quot;${mobileNum}\&quot;,\n    \&quot;country_code\&quot;: \&quot;+91\&quot;,\n    \&quot;otp_token\&quot;: \&quot;1234567\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-id</name>
      <type>Main</type>
      <value>${authid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>auth-token</name>
      <type>Main</type>
      <value>${auth_token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>device-type</name>
      <type>Main</type>
      <value>${devicetype}</value>
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
      <defaultValue>GlobalVariable.authid</defaultValue>
      <description></description>
      <id>182a7e40-6415-4556-a0df-ac942b8914a1</id>
      <masked>false</masked>
      <name>authid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.auth_token</defaultValue>
      <description></description>
      <id>3ccf57a7-5bf7-4070-8072-6f21e0fd22be</id>
      <masked>false</masked>
      <name>auth_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.devicetype</defaultValue>
      <description></description>
      <id>883a9ac9-139f-41a4-969d-2d4fe8dfe9aa</id>
      <masked>false</masked>
      <name>devicetype</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobileNum</defaultValue>
      <description></description>
      <id>2db64922-5315-47b8-8da9-7b417e45dde0</id>
      <masked>false</masked>
      <name>mobileNum</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Host</defaultValue>
      <description></description>
      <id>bbb1b251-2abd-45b2-b60f-b1f60b19e015</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
