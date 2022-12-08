<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TRUECALLER Login</name>
   <tag></tag>
   <elementGuidId>1b63db4b-490d-4205-9f7c-871fb973ad43</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;medium\&quot;: \&quot;truecaller\&quot;,\n    \&quot;payload\&quot;: \&quot;eyJyZXF1ZXN0Tm9uY2UiOiIzNWRiNzc0MS1kNDQ1LTQ4ZjctYWI4NC1lOWVmZWNlMTA3NmUiLCJyZXF1ZXN0VGltZSI6MTU4NzQ1NTAzOSwicGhvbmVOdW1iZXIiOiIrOTE5Nzk5MjQ3MTYxIiwiZmlyc3ROYW1lIjoiUklTSEFCSCIsImxhc3ROYW1lIjoiSmFuZ2lkIiwiZ2VuZGVyIjoiTiIsImNvdW50cnlDb2RlIjoiSU4iLCJlbWFpbCI6InJpc2hhYmhqYW4xMEBnbWFpbC5jb20iLCJ0cnVlTmFtZSI6dHJ1ZSwiYW1iYXNzYWRvciI6ZmFsc2V9\&quot;,\n    \&quot;signature\&quot;: \&quot;TKtUVKEXWdlezaPDqttU4HbFbgy4ZazWmSufwC6x1w4zbt9NIiGRKHnrtTAgMla66Rp3VaH2SepS8YXnUx0w+iZIfRc72891QkAYbWMmMsBAYerJWtVZsg2VP3sVmLIPjXX7WrzOx0JinTv5C2cevXqxo7SIeYxZSuip3OZmjEQ\u003d\&quot;,\n    \&quot;mobile\&quot;: \&quot;${mobile_number}\&quot;,\n    \&quot;imei\&quot;: \&quot;\&quot;,\n    \&quot;full_name\&quot;: \&quot;TEST\&quot;,\n    \&quot;email\&quot;: \&quot;test@gmail.com\&quot;,\n    \&quot;country_code\&quot;: \&quot;+91\&quot;\n} &quot;,
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
      <value>9035902285</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>customer_id</name>
      <type>Main</type>
      <value>155</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>package_id</name>
      <type>Main</type>
      <value></value>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Host}login/</restUrl>
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
      <id>43d6db36-6b94-4fc3-9ea8-2550e364a5d2</id>
      <masked>false</masked>
      <name>Host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Id</defaultValue>
      <description></description>
      <id>4608ec03-b549-46c5-a784-c7b5f7f8a65b</id>
      <masked>false</masked>
      <name>Auth Id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Auth Token</defaultValue>
      <description></description>
      <id>a730e70a-9fca-41e4-8f3a-f3934d7dacf2</id>
      <masked>false</masked>
      <name>Auth Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mobile_number</defaultValue>
      <description></description>
      <id>277adf1a-c5f0-4578-a7e8-c4eb8c76a719</id>
      <masked>false</masked>
      <name>mobile_number</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
