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

WebUI.callTestCase(findTestCase('CMS/Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.refresh()

WebUI.click(findTestObject('CMS/Outlet/MenuOutlet'))

WebUI.waitForElementClickable(findTestObject('CMS/Outlet/Btn_View'), 5)

WebUI.click(findTestObject('CMS/Outlet/Btn_View'))

WebUI.click(findTestObject('CMS/Outlet/UpdateLocation/IndexLocation'))

WebUI.waitForElementVisible(findTestObject('CMS/Outlet/UpdateLocation/Btn_EditLocation'), 5)

WebUI.click(findTestObject('CMS/Outlet/UpdateLocation/Btn_EditLocation'))

WebUI.setText(findTestObject('CMS/Outlet/Label_Address'), 'A')

WebUI.click(findTestObject('CMS/Outlet/UpdateLocation/Btn_SaveLocation'))

WebUI.verifyTextPresent('Outlet location has been successfully updated', true)

WebUI.closeBrowser()

