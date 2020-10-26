import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('CMS/Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('CMS/Member/Menu_Loyalty'))

WebUI.click(findTestObject('CMS/Member/Menu_Member'))

WebUI.delay(2)

WebUI.click(findTestObject('CMS/Member/Add_Member'))

WebUI.setText(findTestObject('CMS/Member/Label_Name'), 'Member Automated')

WebUI.setText(findTestObject('CMS/Member/Label_PhoneNumber'), '82297617354')

WebUI.delay(2)

String Ms = WebUI.verifyTextPresent('Sorry, that phone number already exists :( , please use another phone number.', false)

if (Ms.equals('true')) {
    int RN

    RN = ((Math.random() * 500) as int)

    WebUI.setText(findTestObject('CMS/Member/Label_PhoneNumber'), '' + RN)
}

WebUI.setText(findTestObject('CMS/Member/Label_EmailAddress'), 'rudi@member.id')

String Ms1 = WebUI.verifyTextPresent('Sorry, that email already exists :( , please use another email.', false)

if (Ms1.equals('true')) {
    int RN

    RN = ((Math.random() * 500) as int)

    WebUI.sendKeys(findTestObject('CMS/Member/Label_EmailAddress'), Keys.chord(Keys.SHIFT, Keys.ARROW_UP, Keys.DELETE))

    WebUI.setText(findTestObject('CMS/Member/Label_EmailAddress'), ('rudi' + RN) + '@member.id')
}

WebUI.click(findTestObject('CMS/Member/Label_DOBMember'))

WebUI.click(findTestObject('CMS/Member/DOB_2020'))

WebUI.click(findTestObject('CMS/Member/DOB_Januari'))

WebUI.click(findTestObject('CMS/Member/DOB_1Jan'))

WebUI.click(findTestObject('CMS/Member/Label_Tier'))

WebUI.click(findTestObject('CMS/Member/Label_Tier1'))

WebUI.click(findTestObject('CMS/Member/Btn_Submit'))

WebUI.verifyTextPresent('The new member has been successfully added', true)

