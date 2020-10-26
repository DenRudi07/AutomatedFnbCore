<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateBusinessHourOutlet</name>
   <tag></tag>
   <elementGuidId>53ab47ce-4aa6-4cdc-8f92-304068b72f36</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;openingHours\&quot;: [\n    {\&quot;day\&quot;:\&quot;1\&quot;,\&quot;isAllHour\&quot;: 0, \&quot;isSpecial\&quot;:0, \&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;10:00\&quot;,\&quot;isOpen\&quot;:1},\n    {\&quot;day\&quot;:\&quot;2\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;22:00\&quot;,\&quot;isOpen\&quot;:0},\n    {\&quot;day\&quot;:\&quot;3\&quot;,\&quot;start\&quot;:\&quot;07:00\&quot;,\&quot;end\&quot;:\&quot;21:00\&quot;,\&quot;isOpen\&quot;:1},\n    {\&quot;day\&quot;:\&quot;4\&quot;,\&quot;start\&quot;:\&quot;08:00\&quot;,\&quot;end\&quot;:\&quot;16:00\&quot;,\&quot;isOpen\&quot;:1},\n    {\&quot;day\&quot;:\&quot;5\&quot;,\&quot;start\&quot;:\&quot;09:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n    {\&quot;day\&quot;:\&quot;6\&quot;,\&quot;start\&quot;:\&quot;10:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1},\n    {\&quot;day\&quot;:\&quot;7\&quot;,\&quot;start\&quot;:\&quot;12:00\&quot;,\&quot;end\&quot;:\&quot;23:00\&quot;,\&quot;isOpen\&quot;:1}\n]\n}&quot;,
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
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.API_URL}/v1/outlet/${GlobalVariable.UUID_Outlet}/schedule</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>3f023e1a-9e75-4c60-b3b5-f0296bc9e5bf</id>
      <masked>false</masked>
      <name>Token</name>
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
