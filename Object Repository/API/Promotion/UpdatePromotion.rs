<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdatePromotion</name>
   <tag></tag>
   <elementGuidId>022103da-d4ad-4a29-bd7f-5ff722103697</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;status\&quot;: 1,\n    \&quot;title\&quot;: \&quot;Testing Update Promotion\&quot;,\n    \&quot;details\&quot;: \&quot;testing description update\&quot;,\n    \&quot;tnc\&quot;: \&quot;test tnc update\&quot;,\n    \&quot;howToUse\&quot;: \&quot;How to use test update\&quot;,\n    \&quot;promoCode\&quot;: \&quot;ABC111\&quot;,\n    \&quot;expiredDate\&quot;: \&quot;2020-10-28\&quot;,\n    \&quot;deleteBrand\&quot;: [],\n    \&quot;newBrand\&quot;: [9],\n    \&quot;deleteImage\&quot;: [],\n    \&quot;newImage\&quot;: [\n        {\n            \&quot;type\&quot;: \&quot;PREVIEW\&quot;,\n            \&quot;url\&quot;: \&quot;files/Promotion/PREVIEW-C4TYWNMXKI-144x144.png\&quot;\n        }\n    ]\n}&quot;,
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
   <restUrl>${GlobalVariable.API_URL}/v1/promotion/${GlobalVariable.UUID_Promotion}</restUrl>
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
      <id>e29222b5-0ca3-40f7-8893-804d1d0cbc92</id>
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
